use crate::auth::BackendError;
use crate::secrets::Secret;
use crate::users::User;
use password_auth::{generate_hash, verify_password};
use tokio::task::spawn_blocking;

#[tracing::instrument(level = "info", ret, err)]
pub fn verify_user_password(
    user: Option<User>,
    password_candidate: Secret<String>,
) -> Result<Option<User>, BackendError> {
    // password-based authentication. Compare our form input with an argon2
    // password hash.
    // To prevent timed side-channel attacks, so we always compare the password
    // hash, even if the user doesn't exist.
    return match user {
        // If there is no user with this username we dummy verify the password.
        None => dummy_verify_password(password_candidate),
        Some(user) => {
            // If the user exists, but has no password, we dummy verify the password.
            let Some(password_hash) = user.password_hash.expose() else {
                return dummy_verify_password(password_candidate);
            };

            // If the user exists and has a password, we verify the password.
            match verify_password(password_candidate.expose(), password_hash.as_ref()) {
                Ok(_) => Ok(Some(user)),
                _ => Ok(None),
            }
        }
    };
}

// Prevent side-channel attacks by always verifying the password.
pub fn dummy_verify_password(pw: Secret<impl AsRef<[u8]>>) -> Result<Option<User>, BackendError> {
    let _ = verify_password(
        pw.expose_owned().as_ref(),
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno",
    );
    // Even if the password is correct we still return `Ok(None)`.
    Ok(None)
}

pub async fn hash_password(password: Secret<String>) -> Result<Secret<String>, BackendError> {
    Ok(spawn_blocking(move || Secret::new(generate_hash(password.expose().as_bytes()))).await?)
}
