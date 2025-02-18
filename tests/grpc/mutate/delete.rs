use anyhow::Result;
use sellershut_core::users::{DeleteUserRequest, QueryUserByApIdRequest};
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn delete_user(pool: sqlx::PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;
    let ap_id = String::from("https://example.com/users/johndoe");

    app.mutate
        .delete_user(
            DeleteUserRequest {
                ap_id: ap_id.clone(),
            }
            .into_request(),
        )
        .await?;

    let getter = QueryUserByApIdRequest { ap_id }.into_request();

    let response = app
        .query
        .query_user_by_ap_id(getter)
        .await?
        .into_inner()
        .user;

    assert!(response.is_none());

    Ok(())
}
