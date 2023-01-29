use kvm::monitor_client::MonitorClient;
use kvm::InputRequest;

pub mod kvm {
    tonic::include_proto!("kvm");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MonitorClient::connect("http://[::1]:51151").await?;

    let request = tonic::Request::new(InputRequest { value: 15.into() });

    let response = client.change_input(request).await?;

    println!("Done");

    Ok(())
}
