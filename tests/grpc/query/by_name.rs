use anyhow::Result;
use sellershut_core::users::QueryUserByNameRequest;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn by_name(pool: sqlx::PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let response = app
        .query
        .query_user_by_name(
            QueryUserByNameRequest {
                username: String::from("johndoe"),
                local: Some(true),
            }
            .into_request(),
        )
        .await?
        .into_inner();

    assert!(response.user.is_some());

    Ok(())
}
