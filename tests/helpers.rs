use sellershut_core::users::{
    mutate_users_client::MutateUsersClient, query_users_client::QueryUsersClient,
};
use sellershut_services::{tracing::TracingBuilder, Services};
use sqlx::PgPool;
use tokio::sync::oneshot;
use tonic::transport::Channel;
use users_service::{state::AppState, AppConfig};

use std::sync::Once;

static TRACING: Once = Once::new();

pub struct TestApp {
    state: AppState,
    pub query: QueryUsersClient<Channel>,
    pub mutate: MutateUsersClient<Channel>,
}

impl TestApp {
    pub async fn new(pool: PgPool) -> Self {
        let (tx, rx) = oneshot::channel();
        // Set port to 0 so tests can spawn multiple servers on OS assigned ports.

        // Setup tracing. Once.
        TRACING.call_once(|| {
            TracingBuilder::new().build(Some("warn".into()));
        });

        let services = Services { postgres: pool };

        let app_config = AppConfig {
            something: "into".into(),
        };

        let state = AppState::new(0, services, app_config);

        dbg!(&state.addr.port());

        tokio::spawn(users_service::run(state.clone(), tx));
        let port = rx.await.expect("channel to be open");
        let addr = format!("http://[::1]:{port}");

        let (query_client, mutation_client) = tokio::try_join!(
            QueryUsersClient::connect(addr.to_string()),
            MutateUsersClient::connect(addr)
        )
        .expect("expect server to be running");

        Self {
            state,
            query: query_client,
            mutate: mutation_client,
        }
    }
}
