use log;
use env_logger;
use std::env;
use std::os::unix::net::UnixStream;
use std::net::TcpStream;
use std::io::{*,Write,BufWriter};

mod gnc;
mod poc;
use log::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let log_level = &args[1];
        std::env::set_var("RUST_LOG", log_level);
        println!("Log level = {}", log_level);
    }

    env_logger::init();
    
    println!("Hello, Space! {}-{}-{}","🚀" ,"📡","🛰️");

    let url = "unix:/tmp/space/internal/appcoms";
    error!("{}",url.to_string());
    warn!("{}",url.to_string());
    info!("{}",url.to_string());
    debug!("{}",url.to_string());
    trace!("{}", url.to_string());

    let mut sat = gnc::Satellite::new();
    let coords = sat.get_coordinates();
    
    poc::mult3x3s();
    poc::multVec3xMat3();
/*
    info!("trying connect sender to unix socket @{}-{}.sock", &url[5..]);
    match UnixStream::connect(format!("{}-{}.sock",&url[5..], id)){
        Ok(stream) => {
            info!("sender connected to unix socket @{}-{}.sock", &url[5..]);
            return Box::new(BufWriter::new(stream));
        },
        Err(e) => {
            //FIXME - file exists and permission problem - error and exit
            debug!("Unable to establish connection to @{}-{}.sock due to {}", &url[5..], id, e);
            use std::{thread, time};
            let ten_millis = time::Duration::from_millis(500);
            thread::sleep(ten_millis);
            continue;
        }
    }
*/    
   // sat.fly();

}

