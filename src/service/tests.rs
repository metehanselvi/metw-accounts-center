use super::{AccountService, ServiceResult};
use crate::{dto, repo::impls::MockAccountRepoImpl, util::check_password};

#[tokio::test(flavor = "multi_thread")]
async fn account_creation_mock_mt() -> ServiceResult<()> {
    let repo = MockAccountRepoImpl::boxed_new();

    let account_service = AccountService::new(repo);

    let mut signup_dto = dto::request::Signup {
        username: "user1".to_string(),
        email: "user1@example.com".to_string(),
        password_hash: "paswd1".to_string(),
        keys: dto::request::Keys {
            identity_key: vec![1],
            encrypted_private_key: vec![2],
            encrypted_master_key: vec![3],
        },
    };

    let user1_account_id = account_service.signup(signup_dto.clone()).await?;

    signup_dto.username = "user2".to_string();
    signup_dto.email = "user2@example.com".to_string();
    signup_dto.password_hash = "paswd2".to_string();

    let user2_account_id = account_service.signup(signup_dto).await?;

    let user2_account_id_login = account_service
        .login_with_username(dto::request::LoginWithUsername {
            username: "user2".to_string(),
            password_hash: "paswd2".to_string(),
        })
        .await?;

    let repo = account_service.repo;

    assert!(user2_account_id_login == user2_account_id);

    assert!(repo.get_primary_username(user1_account_id).await?.unwrap() == "user1");
    assert!(
        check_password(
            "paswd1".to_string(),
            repo.get_login_by_username("user1")
                .await?
                .unwrap()
                .password_hash
        )
        .await
    );
    assert!(
        repo.get_login_by_email("user1@example.com")
            .await?
            .is_none()
    );

    assert!(repo.get_primary_username(user2_account_id).await?.unwrap() == "user2");
    assert!(
        repo.get_keys(user2_account_id).await?.unwrap()
            == dto::repo::Keys {
                identity_key: vec![1],
                encrypted_private_key: vec![2],
                encrypted_master_key: vec![3],
            }
    );

    Ok(())
}
