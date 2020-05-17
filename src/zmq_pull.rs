use clap::{App, Arg};

fn cli() -> App<'static, 'static> {
    App::new("ZMQ Pull").arg(
        Arg::with_name("BIND")
            .long("bind-port")
            .takes_value(true)
            .required(false),
    )
}

fn run(bind_port: u16) {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PULL).unwrap();

    socket.set_linger(0).unwrap();
    socket.set_tcp_keepalive(0).unwrap();
    socket.set_rcvtimeo(5000).unwrap();
    socket
        .bind(&format!("tcp://127.0.0.1:{}", bind_port))
        .unwrap();

    loop {
        let msg = match socket.recv_msg(0) {
            Ok(msg) => {
                println!("Processing: {:?}", msg.as_str());
            }
            Err(e) => {
                if e != zmq::Error::EAGAIN {
                    println!("Error receiving message: {:?}", e);
                }
            }
        };
    }
}

fn main() {
    let matches = cli().get_matches();
    let bind = matches.value_of("BIND").unwrap().parse().unwrap();

    println!("ZMQ PULL starting up on port {}", bind);

    run(bind);
}
