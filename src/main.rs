use std::net::TcpListener;

use mailer::configuration::get_configuration;
use mailer::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read config file");

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
