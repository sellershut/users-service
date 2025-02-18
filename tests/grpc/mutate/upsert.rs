use anyhow::Result;
use sellershut_core::users::{QueryUserByApIdRequest, UpsertUserRequest, User};
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn upsert_user(pool: sqlx::PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let mut user = User::default();
    user.username = "test_upsert_username".to_string();
    user.ap_id = "https://example.com/users/johndoe".to_string();

    app.mutate
        .upsert_user(
            UpsertUserRequest {
                user: Some(user.clone()),
            }
            .into_request(),
        )
        .await?;

    let getter = QueryUserByApIdRequest {
        ap_id: user.ap_id.to_string(),
    }
    .into_request();

    let response = app
        .query
        .query_user_by_ap_id(getter)
        .await?
        .into_inner()
        .user
        .unwrap();

    assert_eq!(response.ap_id, user.ap_id);
    assert_eq!(response.username, user.username);

    Ok(())
}
