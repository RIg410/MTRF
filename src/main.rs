use mtrf::ports;
use mtrf::cmd::request::Request;
use mtrf::mtrf::{Mtrf, OnMessage};
use mtrf::cmd::{Mode, CtrRequest, Cmd};
use std::thread;
use mtrf::cmd::response::Response;

pub struct Logger;

impl OnMessage for Logger {
    fn on_message(&mut self, msg: Response) {
        println!("{}", msg);
    }
}

fn main() {
    let port = &ports().unwrap()[0];
    let mut mtrf = Mtrf::new(port, Logger).unwrap();
    dbg!(mtrf.send_request(Request { mode: Mode::RX, ctr: CtrRequest::BindModeOn, cmd: Cmd::Bind, ch: 1, ..Default::default() })
        .unwrap());
    thread::sleep_ms(20000);
}