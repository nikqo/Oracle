use sqlx::postgres::PgPool;
use super::model::DbUser;

pub async fn insert_user(pool: &PgPool, user: &DbUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, discriminator, global_name, avatar, bot, banner, accent_colour)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (id) DO UPDATE
        SET name = $2, discriminator = $3, global_name = $4, avatar = $5, bot = $6, banner = $7, accent_colour = $8
        "#,
        user.id.into(),
        user.name,
        user.discriminator.map(|d| d.get() as i32),
        user.global_name,
        user.avatar,
        user.bot,
        user.banner,
        user.accent_colour.map(|c| c.0 as i64),
    )
    .execute(pool)
    .await?;

    Ok(())
}