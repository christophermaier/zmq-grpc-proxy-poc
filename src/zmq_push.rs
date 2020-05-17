use clap::{App, Arg};
use std::thread;
use std::time;

fn cli() -> App<'static, 'static> {
    App::new("ZMQ Push")
        .arg(Arg::with_name("NAME").long("name").takes_value(true))
        .arg(
            Arg::with_name("CONNECT")
                .long("connect-port")
                .takes_value(true)
                .required(false),
        )
}

fn run(connect_port: u16, name: &str) {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUSH).unwrap();

    socket.set_linger(1000).unwrap();
    socket.set_tcp_keepalive(0).unwrap();
    socket.set_immediate(true).unwrap();
    socket.set_sndhwm(1000).unwrap();
    socket.set_sndtimeo(500).unwrap();
    socket
        .connect(&format!("tcp://127.0.0.1:{}", connect_port))
        .unwrap();

    let mut counter = 0;

    loop {
        counter = counter + 1;
        let message = format!("Message {} from {}", counter, name);
        println!("Sending '{}'", message);
        socket.send(&message, 0);
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn main() {
    let matches = cli().get_matches();
    let connect = matches.value_of("CONNECT").unwrap().parse().unwrap();
    let name = matches.value_of("NAME").unwrap();

    println!(
        "ZMQ PUSH client '{}' starting, sending to port {}",
        name, connect
    );
    run(connect, name);
}
