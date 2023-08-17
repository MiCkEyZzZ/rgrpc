use ::clap::Parser;
use log::log_service_client::LogServiceClient;
use log::LogRequest;

pub mod log {
    tonic::include_proto!("log");
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "лог — простая командная строка для отправки сообщения", long_about = None)]
struct ClientCli {
    #[arg(short = 'с', long = "сервер", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'п', long = "порт", default_value = "50052")]
    port: u16,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();
    let mut client =
        LogServiceClient::connect(format!("http://{}:{}", cli.server, cli.port)).await?;
    let req = tonic::Request::new(LogRequest {
        message: cli.message,
    });
    let res = client.log(req).await?;

    println!("ОТВЕТ={:?}", res.into_inner().message);

    Ok(())
}
