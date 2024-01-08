use mailer::configuration::get_configuration;
use mailer::startup::Application;
use mailer::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("mailer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config file");
    let app = Application::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
