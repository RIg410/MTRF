use std::convert::TryFrom;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::Error;

use crate::cmd::request::Request;
use crate::cmd::response::Response;
use crate::cmd::MESSAGE_LENGTH;
use serial::core::BaudRate::Baud9600;
use serial::core::CharSize::Bits8;
use serial::core::FlowControl::FlowNone;
use serial::core::Parity::ParityNone;
use serial::core::StopBits::Stop1;
use serial::{PortSettings, SerialPort, SystemPort};
use std::io::{Read, Write};

const SETTINGS: PortSettings = PortSettings {
    baud_rate: Baud9600,
    char_size: Bits8,
    parity: ParityNone,
    stop_bits: Stop1,
    flow_control: FlowNone,
};

pub struct Mtrf {
    join: Option<JoinHandle<()>>,
    resp_rx: Receiver<Response>,
    req_tx: Sender<(Request, bool)>,
}

impl Mtrf {
    pub fn new<OnMsg: OnMessage + Send + 'static>(
        port_name: &str,
        on_msg: OnMsg,
    ) -> Result<Mtrf, Error> {
        let mut port = serial::open(port_name)?;
        port.configure(&SETTINGS)?;
        port.set_timeout(Duration::from_millis(20))?;
        let (resp_tx, resp_rx) = channel();

        let (req_tx, req_rx) = channel();
        Ok(Mtrf {
            join: Some(Self::run_loop(port, req_rx, resp_tx, on_msg)),
            resp_rx,
            req_tx,
        })
    }

    fn run_loop<OnMsg: OnMessage + Send + 'static>(
        mut port: SystemPort,
        req_rx: Receiver<(Request, bool)>,
        resp_tx: Sender<Response>,
        mut on_msg: OnMsg,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut wait_ch = None;
            let mut msg = [0; MESSAGE_LENGTH];
            loop {
                match req_rx.try_recv() {
                    Ok((req, wait_resp)) => {
                        debug!("Write request {}", req);
                        if wait_resp {
                            wait_ch = Some(req.ch());
                        }
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

                if let Ok(count) = port.read(&mut msg) {
                    if count == 0 {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    } else if count < MESSAGE_LENGTH {
                        if let Err(err) = port.read_exact(&mut msg[count..]) {
                            error!("Failed to read response. {}", err);
                            break;
                        }
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
                }
            }
        })
    }

    pub fn send(&mut self, req: Request) -> Result<(), Error> {
        self.req_tx.send((req, false))?;
        Ok(())
    }

    pub fn send_request(&mut self, req: Request) -> Result<Response, Error> {
        self.req_tx.send((req, true))?;
        Ok(self.resp_rx.recv()?)
    }
}

pub trait OnMessage {
    fn on_message(&mut self, msg: Response);
}

#[cfg(test)]
pub mod tests {
    use crate::cmd::request::bind;
    use crate::cmd::response::Response;
    use crate::cmd::Mode;
    use crate::mtrf::{Mtrf, OnMessage};
    use crate::ports;
    use std::thread;

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
        mtrf.send_request(bind(Mode::RxF, 0).unwrap()).unwrap();
        thread::sleep_ms(2000);
    }
}
