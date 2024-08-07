use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use tracing::log::SetLoggerError;
use handle_errors::Error;
use crate::models::user::{AuthInfo, User, UserId, UserInfo};
use crate::models::company::{Company, CompanyId};
use crate::models::job::{Job, JobId, JobInfo};
use crate::models::map_resume_job::{NewMapResumeJob, MapResumeJob, MapResumeJobId};
use crate::models::resume::{Resume, ResumeId};
use crate::models::role::{Role, RoleId, RoleInfo};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}
pub trait StoreActionBasic {
    async fn new(db_url: &str) -> Self;
}
impl StoreActionBasic for Store {
    async fn new(db_url: &str) -> Self {
        tracing::warn!("{}", db_url);
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }
}

pub trait MapResumeJobMethods {
    async fn create_map_job_resume(self, new_map_resume_job: NewMapResumeJob) -> Result<MapResumeJob, Error>;
    async fn get_list_job_by_resume(self, resume_id: ResumeId) -> Result<Vec<MapResumeJob>, Error>;
    async fn get_list_resume_by_job_id(self, limit: Option<i32>, offset: i32, job_id: JobId) -> Result<Vec<MapResumeJob>, Error>;
}

impl MapResumeJobMethods for Store {
    async fn create_map_job_resume(self, new_map_resume_job: NewMapResumeJob) -> Result<MapResumeJob, Error>
    {
        match sqlx::query("INSERT INTO map_resume_job (resume_id, job_id) \
                            VALUES ($1, $2)\
                            RETURNING id, resume_id, job_id")
            .bind(new_map_resume_job.resume_id.0)
            .bind(new_map_resume_job.resume_id.0)
            .map(|row: PgRow| MapResumeJob {
                id: Some(MapResumeJobId(row.get("id"))),
                resume_id: ResumeId(row.get("resume_id")),
                job_id: JobId(row.get("job_id"))
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message =
                        error.as_database_error().unwrap().message(),
                    constraint = error
                        .as_database_error()
                        .unwrap()
                        .constraint()
                        .unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
    async fn get_list_job_by_resume(self, resume_id: ResumeId) -> Result<Vec<MapResumeJob>, Error>
    {
        match sqlx::query("SELECT * FROM map_resume_job where resume_id = $1")
            .bind(resume_id.0)
            .map(|row: PgRow| MapResumeJob {
                id: Some(MapResumeJobId(row.get("id"))),
                resume_id: ResumeId(row.get("resume_id")),
                job_id: JobId(row.get("job_id"))
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(list_map) => Ok(list_map),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    async fn get_list_resume_by_job_id(self, limit: Option<i32>, offset: i32, job_id: JobId)
        -> Result<Vec<MapResumeJob>, Error>
    {
        match sqlx::query("SELECT * FROM map_resume_job LIMIT = $1 OFFSET = $2 where job_id = $3")
            .bind(limit)
            .bind(offset)
            .bind(job_id.0)
            .map(|row: PgRow| MapResumeJob {
                id: Some(MapResumeJobId(row.get("id"))),
                resume_id: ResumeId(row.get("resume_id")),
                job_id: JobId(row.get("job_id"))
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(list_map) => Ok(list_map),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}