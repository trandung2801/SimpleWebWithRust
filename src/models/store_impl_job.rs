use crate::models::company::CompanyId;
use crate::models::job::{Job, JobId, NewJob};
use crate::models::store::Store;

use handle_errors::Error;
use sqlx::{
    Row,
};
use sqlx::postgres::PgRow;

pub trait JobStoreMethods {
    async fn create_job(self, new_job: NewJob)
                        -> Result<Job, Error>;
    async fn get_job_by_id(self, job_id: JobId)
                           -> Result<Job, Error>;
    async fn get_list_job(self, limit: Option<i32>, offset: i32)
                          -> Result<Vec<Job>, Error>;
    async fn update_job(self, job: Job)
                        -> Result<Job, Error>;
    async fn delete_job(self, job_id: JobId)
                        -> Result<bool, Error>;
}

impl JobStoreMethods for Store {
    async fn create_job(self, new_job: NewJob)
                            -> Result<Job, Error>
    {
        match sqlx::query("INSERT INTO jobs (job_name, company_id, location, quantity, \
                                                salary, job_level, description, is_delete) \
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)\
                            RETURNING id, job_name, company_id, location, quantity,\
                                        salary, job_level, description, is_delete")
            .bind(new_job.job_name)
            .bind(new_job.company_id.0)
            .bind(new_job.location)
            .bind(new_job.quantity)
            .bind(new_job.salary)
            .bind(new_job.job_level)
            .bind(new_job.description)
            .bind(false)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                job_name:row.get("job_name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                job_level: row.get("job_level"),
                description: row.get("description"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
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

    async fn get_job_by_id(self, job_id: JobId)
                               -> Result<Job, Error>
    {
        match sqlx::query("SELECT * FROM JOBS WHERE id = $1")
            .bind(job_id.0)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                job_name:row.get("job_name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                job_level: row.get("job_level"),
                description: row.get("description"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(job) => Ok(job),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }


    async fn get_list_job(self, limit: Option<i32>, offset: i32)
                              -> Result<Vec<Job>, Error>
    {
        match sqlx::query("SELECT * FROM JOBS LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                job_name:row.get("job_name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                job_level: row.get("job_level"),
                description: row.get("description"),
                is_delete: row.get("is_delete")
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(jobs) => Ok(jobs),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_job(self, job: Job)
                            -> Result<Job, Error>
    {
        match sqlx::query(
            "Update jobs \
                            SET job_name = $1, location = $2, \
                            quantity = $3, salary = $4, job_level= $5, \
                            description = $6 \
                            where id = $7 \
                            RETURNING id, job_name, company_id, location, quantity,\
                                        salary, job_level, description, is_delete")
            .bind(job.job_name)
            .bind(job.location)
            .bind(job.quantity)
            .bind(job.salary)
            .bind(job.job_level)
            .bind(job.description)
            .bind(job.id.unwrap().0)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                job_name:row.get("job_name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                job_level: row.get("job_level"),
                description: row.get("description"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(job) => Ok(job),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_job(self, job_id: JobId)
                            -> Result<bool, Error>
    {
        match sqlx::query("Update resumes set is_delete = $1 where id = $2")
            .bind(true)
            .bind(job_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}