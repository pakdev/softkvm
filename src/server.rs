use kvm::monitor_server::{Monitor, MonitorServer};
use kvm::{InputRequest, InputResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod kvm {
    tonic::include_proto!("kvm");
}

#[derive(Debug, Default)]
pub struct MyMonitor {}

#[tonic::async_trait]
impl Monitor for MyMonitor {
    async fn change_input(
        &self,
        request: Request<InputRequest>,
    ) -> Result<Response<InputResponse>, Status> {
        Ok(Response::new(InputResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:51151".parse()?;
    let monitor = MyMonitor::default();

    Server::builder()
        .add_service(MonitorServer::new(monitor))
        .serve(addr)
        .await?;

    Ok(())
}
