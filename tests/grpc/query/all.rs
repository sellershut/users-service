use anyhow::Result;
use sellershut_core::google;
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn check_all(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let response = app
        .query
        .query_users(google::protobuf::Empty::default().into_request())
        .await?
        .into_inner()
        .users;

    dbg!(&response);

    // 7 inserts in fixtures, but only 5 are local
    assert_eq!(response.len(), 5);

    Ok(())
}
