use clap::{App, Arg};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

fn cli() -> App<'static, 'static> {
    App::new("GRPC Server").arg(
        Arg::with_name("BIND")
            .help("Port the GRPC server should bind")
            .long("bind-port")
            .takes_value(true)
            .required(true),
    )
}

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

async fn run(bind: u16) -> Result<(), Box<dyn std::error::Error>> {
    let greeter = MyGreeter::default();
    Ok(Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), bind))
        .await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let bind: u16 = matches.value_of("BIND").unwrap().parse().unwrap();

    println!("GRPC Server starting up on port {}", bind);

    run(bind).await;
    Ok(())
}
