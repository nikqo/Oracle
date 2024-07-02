use sqlx::postgres::PgPool;
use super::model::DbUser;

// Crud Operations for users in model.rs

pub async fn create_user(pool: &PgPool, user: &DbUser) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        r#"
        INSERT INTO users (id, name, discriminator, global_name, avatar, bot)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id) DO UPDATE
        SET name = $2, discriminator = $3, global_name = $4, avatar = $5, bot = $6
        "#,
        user.id.into(),
        user.name,
        user.discriminator.map(|d| d.get() as i32),
        user.global_name,
        user.avatar,
        user.bot,
    )
    .execute(pool)
    .await {
        Ok(_) => {
            println!("User: {} created in database", user.name);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            Err(e)
        },
    }
}

pub async fn fetch_user(pool: &PgPool, user: DbUser) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT * FROM users
        WHERE id = $1
        "#,
        user.id.into(),
    )
    .fetch_optional(pool)
    .await {
        Ok(Some(_)) => {
            println!("User: {} found in database", user.name);
            Ok(())
        }
        Ok(None) => {
            println!("User {} not found in database", user.name);
            if let Err(e) = create_user(&pool, &user).await {
                eprintln!("Error creating user: {:?}", e);
                return Err(e)
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Error fetching user: {:?}", e);
            Err(e)
        }
    }
}

pub async fn update_user(pool: PgPool, user: DbUser) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE id = $1
        "#,
        user.id.into()
    )
    .fetch_optional(&pool)
    .await {
        Ok(Some(_)) => {
            match sqlx::query!(
                r#"
                UPDATE users
                SET name = $2, discriminator = $3, global_name = $4, avatar = $5, bot = $6
                WHERE id = $1
                "#,
                user.id.into(),
                user.name,
                user.discriminator.map(|d| d.get() as i32),
                user.global_name,
                user.avatar,
                user.bot,
            )
            .execute(&pool)
            .await {
                Ok(_) => {
                    println!("User: {} updated in database", user.name);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("Error updating user: {:?}", e);
                    Err(e)
                }
            }
        }
        Ok(None) => {
            println!("User: {} not found in database", user.name);
            if let Err(e) = create_user(&pool, &user).await {
                eprintln!("Error creating user: {:?}", e);
                return Err(e)
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error fetching user: {:?}", e);
            Err(e)
        }
    }
}

pub async fn delete_user(pool: PgPool, user: DbUser) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE id = $1
        "#,
        user.id.into()
    )
    .fetch_optional(&pool)
    .await {
        Ok(Some(_)) => {
            match sqlx::query!(
                r#"
                DELETE FROM users
                WHERE id = $1
                "#,
                user.id.into()
            )
            .execute(&pool)
            .await {
                Ok(_) => {
                    println!("User: {} deleted", user.name);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error deleting user: {:?}", e);
                    Err(e)
                }
            }
        }
        Ok(None) => {
            println!("User: {} not found in database", user.name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error fetching user: {:?}", e);
            Err(e)
        }
    }
}
