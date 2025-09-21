use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::{models::{CreateEvent, CreateUser, GetEvent, GetEventViews, UpdateEvent, UpdateUser, ViewEvent}};

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
        let created_at = time::OffsetDateTime::now_utc();
        let format = time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]"
        );
        let start_date = time::OffsetDateTime::parse(&create_event.start_date, format).unwrap();
        info!("Parsed start_date: {:?}", start_date);
        let end_date =  {
            let end = create_event.end_date.as_str();
            if end.is_empty() {
                None
            } else {
                Some(time::OffsetDateTime::parse(end, format).unwrap())
            }
        };
        let user_id = uuid::Uuid::parse_str(&create_event.user_id).unwrap();
        sqlx::query!(
            r#"
            INSERT INTO events (id, title, description, created_at, address, image_url, category,start_date,end_date,total_places,user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            id,
            create_event.title,
            create_event.description,
            created_at,
            create_event.address,
            create_event.image_url,
            create_event.category,
            start_date,
            end_date,
            create_event.places,
            user_id
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
                start_date = COALESCE($8, start_date),
                end_date = COALESCE($9, end_date)
            WHERE id = $1
            "#,
            update_event.id,
            update_event.title,
            update_event.description,
            update_event.address,
            update_event.image_url,
            update_event.category,
            update_event.public,
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

    pub async fn get_events(&self) -> Result<Vec<GetEvent>, sqlx::Error> {
        let mut events = vec![];
        let _event = sqlx::query!(
            r#"
            SELECT id, title, description, created_at, address, image_url, category,public, user_id,start_date,end_date,total_places FROM events
            "#
        )
        .fetch_all(&self.db)
        .await?;
        for event in &_event {
            let get_views = self.get_viewed_event(event.id).await?;
            let event = GetEvent {
                id: event.id,
                title: event.title.clone(),
                description: event.description.clone(),
                created_at: event.created_at,
                address: event.address.clone(),
                image_url: event.image_url.clone(),
                category: event.category.clone(),
                public: event.public,
                user_id: event.user_id,
                views: get_views,
                start_date: event.start_date,
                end_date: event.end_date,
                total_places: event.total_places,
            };
            events.push(event);
        }

        Ok(events)
    }

    pub async fn view_event(&self, view:ViewEvent) -> Result<(), sqlx::Error> {
        let now = time::OffsetDateTime::now_utc();
        let viewed_at = time::PrimitiveDateTime::new(now.date(), now.time());
        sqlx::query!(
            r#"
            INSERT INTO event_views (event_id,user_id, viewed_at)
            VALUES ($1, $2, $3)
            "#,
            view.event_id,
            view.user_id,
            viewed_at
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn get_viewed_event(
        &self,
        event_id: uuid::Uuid,
    ) -> Result<GetEventViews, sqlx::Error> {
        let events = sqlx::query_as!(
            GetEventViews,
            r#"
            SELECT COUNT(*) as views  FROM event_views WHERE event_id = $1
            "#,
            event_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(events)
    }
}
