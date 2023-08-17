use tonic::{transport::Server, Request, Response, Status};

use log::log_service_server::{LogService, LogServiceServer};
use log::{LogRequest, LogResponse};

use ::clap::Parser;

pub mod log {
    tonic::include_proto!("log");
}

#[derive(Debug, Default)]
pub struct Log {}

#[tonic::async_trait]
impl LogService for Log {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        println!("Получил запрос: {:?}", request);

        let res = LogResponse {
            message: format!("{}", request.into_inner().message),
        };

        Ok(Response::new(res))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "лог-сервер", long_about = None)]
struct ServeCli {
    #[arg(short = 'с', long = "сервер", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'п', long = "порт", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServeCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let log = Log::default();

    println!("Сервер запущен на порту: {}", addr);

    Server::builder()
        .add_service(LogServiceServer::new(log))
        .serve(addr)
        .await?;

    Ok(())
}
