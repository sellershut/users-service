use sellershut_core::users::{
    mutate_users_server::MutateUsersServer, query_users_server::QueryUsersServer,
};

use tonic::transport::{server::TcpIncoming, Server};
use tracing::info;

use crate::state::AppState;

pub async fn serve(state: AppState, tx: tokio::sync::oneshot::Sender<u16>) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(state.addr).await?;

    let socket_addr = listener
        .local_addr()
        .expect("should get socket_addr from listener");

    tx.send(socket_addr.port())
        .expect("port channel to be open");

    info!(addr = ?socket_addr, "starting server");

    Server::builder()
        .trace_fn(|_| tracing::info_span!(env!("CARGO_PKG_NAME")))
        .add_service(QueryUsersServer::new(state.clone()))
        .add_service(MutateUsersServer::new(state))
        .serve_with_incoming(TcpIncoming::from_listener(listener, true, None).expect("listener"))
        .await?;

    Ok(())
}
