use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use anyhow::Error;
use serialport;
use serialport::SerialPort;

use crate::cmd::{MESSAGE_LENGTH, REQUEST_ST};
use crate::cmd::request::Request;
use crate::cmd::response::Response;
use crate::PortInfo;

pub struct Mtrf {
    port: Box<dyn SerialPort>,
    join: Option<JoinHandle<()>>,
    resp_rx: Receiver<Response>,
}

impl Mtrf {
    pub fn new<OnMsg: OnMessage + Send + 'static>(port: &PortInfo, on_msg: OnMsg) -> Result<Mtrf, Error> {
        let port = serialport::new(&port.port_name, 9600).open()?;
        let (tx, rx) = channel();
        let read_port = port.try_clone()?;
        Ok(Mtrf { port, join: Some(Self::run_loop(read_port, tx, on_msg)), resp_rx: rx })
    }

    fn run_loop<OnMsg: OnMessage + Send + 'static>(mut port: Box<dyn SerialPort>, tx: Sender<Response>, on_msg: OnMsg) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let mut msg = [0; MESSAGE_LENGTH];
                if let Err(err) = port.read_exact(&mut msg) {
                    warn!("Failed to read message from port {}", err);
                    break;
                }

                if msg[0] == REQUEST_ST {
                    on_msg.on_message(Request::from(msg));
                } else {
                    if let Err(err) = tx.send(Response::from(msg)) {
                        warn!("Failed to send response: {}", err);
                        break;
                    }
                }
            }
        })
    }

    pub fn send_request(&mut self, req: Request) -> Result<Response, Error> {
        self.port.write_all(req.as_ref())?;
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
    fn on_message(&self, msg: Request);
}

#[cfg(test)]
pub mod tests {
    use crate::cmd::request::{Request, set_mod};
    use crate::mtrf::{Mtrf, OnMessage};
    use crate::ports;
    use crate::cmd::Mode;

    pub struct Logger;

    impl OnMessage for Logger {
        fn on_message(&self, msg: Request) {
            println!("{}", msg);
        }
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn bind_and_read() {
        init();
        let port = &ports().unwrap()[0];
        let mut mtrf = Mtrf::new(port, Logger).unwrap();

        dbg!(mtrf.send_request(set_mod(Mode::Service)).unwrap());

    }
}
