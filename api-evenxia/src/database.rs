use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::models::{CreateEvent, CreateUser, GetEvent, UpdateEvent, UpdateUser};

#[derive(Debug, Clone)]
pub struct DB {
    pub db: PgPool,
}
impl DB {
    pub async fn connect(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new().connect(db_url).await?;
        Ok(Self { db })
    }

    pub async fn create_user(&self, user: &CreateUser) -> Result<(), sqlx::Error> {
        let id = uuid::Uuid::now_v7();
        let password = hash(&user.password, DEFAULT_COST).unwrap();
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password, age, firstname, lastname, address, postalcode, city, number_phone)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            id,
            user.username,
            user.email,
            password,
            user.age,
            user.firstname,
            user.lastname,
            user.address,
            user.postalcode,
            user.city,
            user.number_phone
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
    pub async fn verify_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<uuid::Uuid, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT password,id FROM users WHERE email = $1 OR username = $2
            "#,
            email,
            username
        )
        .fetch_optional(&self.db)
        .await?;
        info!("Record found: {:?}", record);
        if let Some(row) = record {
            match verify(&password, &row.password) {
                Ok(true) => Ok(row.id),
                Ok(false) => Err(sqlx::Error::RowNotFound),
                Err(_) => Err(sqlx::Error::RowNotFound),
            }
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn get_user(&self, user_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let _user = sqlx::query!(
            r#"
            SELECT id, username, email FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn update_user(&self, update_user: UpdateUser) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users SET 
                username = COALESCE($2, username),
                email = COALESCE($3, email),
                password = COALESCE($4, password),
                age = COALESCE($5, age),
                firstname = COALESCE($6, firstname),
                lastname = COALESCE($7, lastname),
                address = COALESCE($8, address),
                postalcode = COALESCE($9, postalcode),
                city = COALESCE($10, city),
                number_phone = COALESCE($11, number_phone)
            WHERE id = $1
            "#,
            update_user.id,
            update_user.username,
            update_user.email,
            update_user.password,
            update_user.age,
            update_user.firstname,
            update_user.lastname,
            update_user.address,
            update_user.postalcode,
            update_user.city,
            update_user.number_phone
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn create_event(&self, create_event: CreateEvent) -> Result<(), sqlx::Error> {
        let id = uuid::Uuid::now_v7();
        let date = time::OffsetDateTime::now_utc();
        // Convert OffsetDateTime to chrono::DateTime<Utc>
        sqlx::query!(
            r#"
            INSERT INTO events (id, title, description, date, address, image_url, category,city,start_date,end_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            id,
            create_event.title,
            create_event.description,
            date,
            create_event.address,
            create_event.image_url,
            create_event.category,
            create_event.city,
            create_event.start_date,
            create_event.end_date,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn update_event(&self, update_event: UpdateEvent) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE events SET 
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                address = COALESCE($4, address),
                image_url = COALESCE($5, image_url),
                category = COALESCE($6, category),
                "public" = COALESCE($7, "public"),
                city = COALESCE($8, city),
                start_date = COALESCE($9, start_date),
                end_date = COALESCE($10, end_date)
            WHERE id = $1
            "#,
            update_event.id,
            update_event.title,
            update_event.description,
            update_event.address,
            update_event.image_url,
            update_event.category,
            update_event.public,
            update_event.city,
            update_event.start_date,
            update_event.end_date
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_event(&self, event_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM events WHERE id = $1
            "#,
            event_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn get_event(&self, event_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let _event = sqlx::query_as!(
            GetEvent,
            r#"
            SELECT id, title, description, date, address, image_url, category,public, id_user,start_date,end_date,city FROM events WHERE id = $1
            "#,
            event_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(())
    }
}
