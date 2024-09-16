use std::net::TcpListener;

use zero2prod::configuration::{self, get_configuration};
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{}", port);
    run(listener)?.await
}
