use std::convert::TryFrom;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::Error;
use serialport;
use serialport::{DataBits, SerialPort, StopBits};

use crate::cmd::{MESSAGE_LENGTH};
use crate::cmd::request::Request;
use crate::cmd::response::Response;
use crate::PortInfo;

pub struct Mtrf {
    port: Box<dyn SerialPort>,
    join: Option<JoinHandle<()>>,
    resp_rx: Receiver<Response>,
    req_tx: Sender<Request>,
}

impl Mtrf {
    pub fn new<OnMsg: OnMessage + Send + 'static>(port: &PortInfo, on_msg: OnMsg) -> Result<Mtrf, Error> {
        let port_builder = serialport::new(&port.port_name, 9600)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .timeout(Duration::from_secs(u64::MAX));

        let port = port_builder.open()?;

        let (resp_tx, resp_rx) = channel();

        let (req_tx, req_rx) = channel();
        let read_port = port.try_clone()?;
        Ok(Mtrf { port, join: Some(Self::run_loop(read_port, req_rx, resp_tx, on_msg)), resp_rx, req_tx })
    }

    fn run_loop<OnMsg: OnMessage + Send + 'static>(mut port: Box<dyn SerialPort>, req_rx: Receiver<Request>, resp_tx: Sender<Response>, on_msg: OnMsg) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut wait_ch = None;
            loop {
                match req_rx.try_recv() {
                    Ok(req) => {
                        debug!("Write request {}", req);
                        wait_ch = Some(req.ch());
                        if let Err(err) = port.write_all(&req.to_message()) {
                            warn!("Failed to write request {}", err);
                            break;
                        }
                    }
                    Err(TryRecvError::Empty) => {
                        // no-op
                    }
                    Err(TryRecvError::Disconnected) => {
                        info!("Request channel disconnected");
                        break;
                    }
                }

                let bytes = match port.bytes_to_read() {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        error!("Failed to read response {}", err);
                        break;
                    }
                };

                if bytes >= 17 {
                    let mut msg = [0; MESSAGE_LENGTH];
                    if let Err(err) = port.read_exact(&mut msg) {
                        error!("Failed to read response {}", err);
                        break;
                    }

                    match Response::try_from(msg) {
                        Ok(resp) => {
                            debug!("Receive msg:{:?}", resp);
                            if Some(resp.ch) == wait_ch {
                                if let Err(err) = resp_tx.send(resp) {
                                    error!("Failed to send response: {:?}", err);
                                    break;
                                }
                                wait_ch = None;
                            } else {
                                on_msg.on_message(resp);
                            }
                        }
                        Err(err) => {
                            warn!("Failed to decode response {}: msg:[{:?}]", err, msg);

                        }
                    }
                } else {
                    thread::sleep(Duration::from_millis(5));
                }
            }
        })
    }

    pub fn send_request(&mut self, req: Request) -> Result<Response, Error> {
        self.req_tx.send(req)?;
        Ok(self.resp_rx.recv()?)
    }
}

impl Drop for Mtrf {
    fn drop(&mut self) {
        if let Err(err) = self.port.set_break() {
            warn!("Failed to drop Mtrf: {}", err);
        } else {
            if let Some(join) = self.join.take() {
                if let Err(err) = join.join() {
                    warn!("Failed to drop Mtrf: {:?}", err);
                }
            }
        }
    }
}

pub trait OnMessage {
    fn on_message(&self, msg: Response);
}

#[cfg(test)]
pub mod tests {
    use crate::cmd::Mode;
    use crate::cmd::request::bind;
    use crate::mtrf::{Mtrf, OnMessage};
    use crate::ports;
    use crate::cmd::response::Response;

    pub struct Logger;

    impl OnMessage for Logger {
        fn on_message(&self, msg: Response) {
            println!("{}", msg);
        }
    }

    #[test]
    pub fn bind_and_read() {
        let port = &ports().unwrap()[0];
        let mut mtrf = Mtrf::new(port, Logger).unwrap();
        mtrf.send_request(bind(Mode::TxF, 2).unwrap()).unwrap();
    }
}
