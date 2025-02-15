use anyhow::Result;
use sellershut_core::users::QueryUsersFollowingRequest;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn followers(pool: sqlx::PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let response = app
        .query
        .query_user_following(
            QueryUsersFollowingRequest {
                id: String::from("https://example.com/users/user_1"),
            }
            .into_request(),
        )
        .await?
        .into_inner();

    assert_eq!(response.users.len(), 2);

    Ok(())
}
