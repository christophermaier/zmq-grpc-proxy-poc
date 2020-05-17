use clap::{App, Arg};
use futures::future::try_join;
use futures::FutureExt;
use std::net::Ipv4Addr;
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::stream::StreamExt;

fn cli() -> App<'static, 'static> {
    App::new("Proxy")
        .arg(
            Arg::with_name("BIND")
                .help("Port the proxy should bind")
                .long("bind-port")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("ZMQ")
                .help("Port the ZMQ backend is listening on")
                .long("zmq-port")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("GRPC")
                .help("Port the GRPC backend is listening on")
                .long("grpc-port")
                .takes_value(true)
                .required(true),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let bind: u16 = matches.value_of("BIND").unwrap().parse().unwrap();
    let zmq: u16 = matches.value_of("ZMQ").unwrap().parse().unwrap();
    let grpc: u16 = matches.value_of("GRPC").unwrap().parse().unwrap();

    println!("Proxy starting up on port {}", bind);
    println!("--> Forwarding ZMQ traffic to {}", zmq);
    println!("--> Forwarding GRPC traffic to {}", grpc);

    run(bind, zmq, grpc).await;
    Ok(())
}

async fn run(bind: u16, zmq_port: u16, grpc_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind((Ipv4Addr::LOCALHOST, bind)).await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(mut stream) => {
                println!("A new connection to proxy!");
                let mut head: StreamHead = [0; 14];

                // Peek at the data
                stream.peek(&mut head).await?;

                match Backend::decide(&head) {
                    Backend::GRPC => {
                        println!("--> Forwarding to GRPC backend");

                        let transfer = transfer(stream, grpc_port).map(|r| {
                            if let Err(e) = r {
                                println!("Failed to transfer; error={}", e);
                            }
                        });
                        tokio::spawn(transfer);
                    }
                    Backend::ZMQ => {
                        println!("--> Forwarding to ZMQ backend");
                        let transfer = transfer(stream, zmq_port).map(|r| {
                            if let Err(e) = r {
                                println!("Failed to transfer; error={}", e);
                            }
                        });
                        tokio::spawn(transfer);
                    }
                }
            }
            Err(e) => { /* connection failed */ }
        }
    }
    Ok(())
}

type StreamHead = [u8; 14];

enum Backend {
    ZMQ,
    GRPC,
}

impl Backend {
    fn decide(head: &StreamHead) -> Backend {
        if std::str::from_utf8(head).is_ok() {
            Backend::GRPC
        } else {
            Backend::ZMQ
        }
    }
}

// https://github.com/tokio-rs/tokio/blob/master/examples/proxy.rs
async fn transfer(mut inbound: TcpStream, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut outbound = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = io::copy(&mut ri, &mut wo);
    let server_to_client = io::copy(&mut ro, &mut wi);

    try_join(client_to_server, server_to_client).await?;

    Ok(())
}
