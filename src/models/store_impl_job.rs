use crate::models::company::CompanyId;
use crate::models::job::{JobInfo, Job, JobId};
use crate::models::store::Store;

use handle_errors::Error;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

pub trait JobStoreMethods {
    async fn create_job(self, new_job: JobInfo)
                        -> Result<Job, Error>;
    async fn get_job_by_id(self, job_id: JobId)
                           -> Result<Job, Error>;
    async fn get_job_by_company_id(self, company_id: CompanyId)
                                   -> Result<Job, Error>;
    async fn get_list_job(self)
                          -> Result<Vec<Job>, Error>;
    async fn update_job(self, job_info: JobInfo)
                        -> Result<Job, Error>;
    async fn delete_job(self, job_id: JobId)
                        -> Result<bool, Error>;
}

impl JobStoreMethods for Store {
    async fn create_job(self, new_job: JobInfo)
                            -> Result<Job, Error>
    {
        match sqlx::query("INSERT INTO jobs (name, company_id, location, quantity, \
                                                salary, lever, description, is_delete) \
                            VALUES ($1, $2, $3, $4, S5, $6, $7, $8)\
                            RETURNING id, name, company_id, location, quantity,\
                                        salary, lever, description, is_delete")
            .bind(new_job.name)
            .bind(new_job.company_id.0)
            .bind(new_job.location)
            .bind(new_job.quantity)
            .bind(new_job.salary)
            .bind(new_job.level)
            .bind(new_job.description)
            .bind(false)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                name:row.get("name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                level: row.get("level"),
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
        match sqlx::query("SELECT * FROM RESUMES WHERE id = $1")
            .bind(job_id.0)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                name:row.get("name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                level: row.get("level"),
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

    async fn get_job_by_company_id(self, company_id: CompanyId)
                                       -> Result<Job, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES WHERE company_id = $1")
            .bind(company_id.0)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                name:row.get("name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                level: row.get("level"),
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

    async fn get_list_job(self)
                              -> Result<Vec<Job>, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES")
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                name:row.get("name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                level: row.get("level"),
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

    async fn update_job(self, job_info: JobInfo)
                            -> Result<Job, Error>
    {
        match sqlx::query(
            "Update jobs SET (name, company_id, location, quantity, \
                                                salary, lever, description) \
                            VALUES ($1, $2, $3, $4, S5, $6, $7)\
                            RETURNING id, name, company_id, location, quantity,\
                                        salary, lever, description, is_delete")
            .bind(job_info.name)
            .bind(job_info.company_id.0)
            .bind(job_info.location)
            .bind(job_info.quantity)
            .bind(job_info.salary)
            .bind(job_info.level)
            .bind(job_info.description)
            .map(|row: PgRow| Job {
                id: Some(JobId(row.get("id"))),
                name:row.get("name"),
                company_id: CompanyId(row.get("company_id")),
                location: row.get("location"),
                quantity: row.get("quantity"),
                salary: row.get("salary"),
                level: row.get("level"),
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