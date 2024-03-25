use sqlx::PgPool;

#[sqlx::test]
async fn get_users_test(pool: PgPool) -> sqlx::Result<()> {
    Ok(())
}
