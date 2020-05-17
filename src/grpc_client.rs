use clap::{App, Arg};
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::{thread, time};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

fn cli() -> App<'static, 'static> {
    App::new("GRPC Client")
        .arg(Arg::with_name("NAME").long("name").takes_value(true))
        .arg(
            Arg::with_name("CONNECT")
                .long("connect-port")
                .takes_value(true)
                .required(true),
        )
}

async fn run(connect: u16, name: &str) {
    let mut client = GreeterClient::connect(format!("http://127.0.0.1:{}", connect))
        .await
        .unwrap();
    let mut counter = 0;

    loop {
        counter = counter + 1;
        let request = tonic::Request::new(HelloRequest {
            name: name.into(),
            count: counter,
        });
        println!("Sending '{:?}'", request);
        let response = client.say_hello(request).await.unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let connect = matches.value_of("CONNECT").unwrap().parse().unwrap();
    let name = matches.value_of("NAME").unwrap();

    run(connect, name).await;
    Ok(())
}
