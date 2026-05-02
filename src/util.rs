use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

/// Argon2-hashed password.
pub async fn check_password(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || {
        let parsed_hash = if let Ok(parsed_hash) = PasswordHash::new(&hash) {
            parsed_hash
        } else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    })
    .await
    .unwrap()
}

/// Do Argon2 hasing on the password.
pub async fn hash_password(password: String) -> String {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    })
    .await
    .unwrap()
}

#[cfg(test)]
#[tokio::test]
async fn test_hash() {
    let password = "very_very_strong_password";

    let hash = hash_password(password.to_string()).await;

    assert!(check_password(password.to_string(), hash).await);

    assert!(
        !check_password(
            password.to_string(),
            "tring_to_attack_with_inalid_hash_string".to_string()
        )
        .await
    );
}
