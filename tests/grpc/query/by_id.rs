use anyhow::Result;
use sellershut_core::users::QueryUserByApIdRequest;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn by_id(pool: sqlx::PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let query_user_by_id = QueryUserByApIdRequest {
        ap_id: String::from("https://example.com/users/johndoe"),
    };
    let response = app
        .query
        .query_user_by_ap_id(query_user_by_id.into_request())
        .await?
        .into_inner()
        .user;

    assert!(response.is_some());

    Ok(())
}
