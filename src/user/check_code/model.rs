use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow};
use anyhow::Result;
use async_trait::async_trait;
use crate::crud::{CRUD, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct CheckCode {
    code: String,
    owner: String,
}

impl Responder for CheckCode {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

#[async_trait]
#[allow(unused_variables)]
impl CRUD for CheckCode {
    type KeyType = String;
    type RequestType = CheckCode;

    async fn create(r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO check_code (code, owner) VALUES ($1, $2)")
            .bind(&r.code)
            .bind(&r.owner)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(Status::ok())
    }

    async fn read(pool: &PgPool) -> Result<Vec<Self>> {
        let mut data = vec![];

        let recs = sqlx::query!(
            r#"
                SELECT * FROM check_code
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            data.push(CheckCode {
                code: rec.code,
                owner: rec.owner
            });
        }

        Ok(data)
    }

    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM check_code WHERE code = $1
                "#,
                &key
            )
            .fetch_one(&*pool)
            .await?;

        Ok(CheckCode {
            code: rec.code,
            owner: rec.owner
        })
    }

    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        unimplemented!()
    }

    async fn delete(key: Self::KeyType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("DELETE FROM check_code WHERE code = $1")
            .bind(&key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }
}