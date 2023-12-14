use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use mailer::configuration::get_configuration;
use mailer::startup::run;
use mailer::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("mailer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config file");
    let pool_conn = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address)?;

    run(listener, pool_conn)?.await?;
    Ok(())
}
