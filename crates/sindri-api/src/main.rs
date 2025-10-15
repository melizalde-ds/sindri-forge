use sindri_api::{create_app, create_socket_client};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket_client = Arc::new(create_socket_client(String::from("/tmp/sindri.sock")));
    let app = create_app(socket_client);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}
