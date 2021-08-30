use mtrf::cmd::request::Request;
use mtrf::cmd::response::Response;
use mtrf::mtrf::{Mtrf, OnMessage};
use std::thread;

pub struct Logger;

impl OnMessage for Logger {
    fn on_message(&mut self, msg: Response) {
        println!("{}", msg);
    }
}

fn main() {
    let mut mtrf = Mtrf::new("/dev/tty.usbserial-AL065KM0", Logger).unwrap();
    // dbg!(mtrf.send_request(Request { mode: Mode::RX, ctr: CtrRequest::BindModeOn, cmd: Cmd::Bind, ch: 1, ..Default::default() })
    //     .unwrap());
    thread::sleep_ms(20000);
}
