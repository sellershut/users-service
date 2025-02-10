use anyhow::Result;
use sellershut_core::users::{FollowUserRequest, QueryUserByIdRequest};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn follow_user(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let base_id = String::from("https://example.com/users/johndoe");
    let follow = String::from("https://other.com/users/follower");

    app.mutate
        .follow_user(
            FollowUserRequest {
                follow_url: follow.clone(),
                ap_id: base_id.clone(),
            }
            .into_request(),
        )
        .await?;

    let getter = QueryUserByIdRequest {
        id: base_id.to_string(),
    }
    .into_request();

    let response = app
        .query
        .query_user_by_id(getter)
        .await?
        .into_inner()
        .user
        .unwrap();

    assert!(response.followers.contains(&follow));

    Ok(())
}
