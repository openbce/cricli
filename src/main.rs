mod rpc;

use self::crirpc::runtime_service_client::RuntimeServiceClient;
use crate::rpc::cri as crirpc;

use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let path = "/run/containerd/containerd.sock";

            // Connect to a Uds socket
            UnixStream::connect(path)
        }))
        .await?;

    let request = crirpc::VersionRequest {
        version: "*".to_string(),
    };

    let mut host_client = RuntimeServiceClient::new(channel);
    let version = host_client.version(request.clone()).await?;

    let resp = version.into_inner();

    println!("RESPONSE={:?}", resp);

    Ok(())
}
