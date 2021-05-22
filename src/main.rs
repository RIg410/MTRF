use mtrf::ports;
use mtrf::mtrf::{Mtrf, OnMessage};
use mtrf::cmd::request::{bind, set_mode, Request};
use mtrf::cmd::{Mode, Cmd};
use std::thread;
use mtrf::cmd::response::Response;


pub struct Logger;

impl OnMessage for Logger {
    fn on_message(&self, msg: Response) {
        println!("msg---{}", msg);
    }
}

pub fn main() {
    env_logger::builder().try_init().unwrap();
    let port = &ports().unwrap()[0];
    let mut mtrf = Mtrf::new(port, Logger).unwrap();
    thread::sleep_ms(3000);
    println!("send");
     println!("set mode {:?}", mtrf.send_request(set_mode(Mode::Service)).unwrap());
     println!("bind {:?}", mtrf.send_request(bind(Mode::TxF, 4).unwrap()).unwrap());

    // let mut req = Request::default();
    // req.mode = Mode::Service;
    // req.cmd = Cmd::ClearMemory;
    // mtrf.send_request(req).unwrap();
    // println!("clear memory {:?}", mtrf.send_request(req).unwrap());

    thread::sleep_ms(100000000);
}

