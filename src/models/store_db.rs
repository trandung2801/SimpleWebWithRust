use std::time::Duration;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use tracing::{event, Level};
use async_trait::async_trait;
use crate::service::handle_errors::Error;
use crate::models::map_resume_job::{MapResumeJob, MapResumeJobId, NewMapResumeJob};
use crate::models::user::{AuthInfo, User, UserId, UserInfo};
use crate::models::role::{RoleInfo, RoleId, Role, USER_ROLE_ID};
use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::job::{Job, JobId, NewJob};
use crate::models::resume::{Resume, ResumeId, NewResume};
use crate::models::store_trait::StoreMethods;

#[derive(Debug, Clone)]
pub struct DatabaseStore {
    pub connection: PgPool,
}
impl DatabaseStore {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(100)
            // .connect_timeout(Duration::from_secs(60))
            .connect(db_url)
            .await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {} with url {}", e, db_url),
        };
        DatabaseStore {
            connection: db_pool,
        }
    }
}

#[async_trait]
impl StoreMethods for DatabaseStore {
    async fn create_map_job_resume(&self, new_map_resume_job: NewMapResumeJob)
        -> Result<MapResumeJob, Error>
    {
        match sqlx::query("INSERT INTO map_resume_job (resume_id, job_id) \
                            VALUES ($1, $2)\
                            RETURNING id, resume_id, job_id")
            .bind(new_map_resume_job.resume_id.0)
            .bind(new_map_resume_job.job_id.0)
            .map(|row: PgRow| MapResumeJob {
                id: Some(MapResumeJobId(row.get("id"))),
                resume_id: ResumeId(row.get("resume_id")),
                job_id: JobId(row.get("job_id"))
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Create map_job_resume from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    async fn get_list_job_by_resume(&self, resume_id: ResumeId)
        -> Result<Vec<MapResumeJob>, Error>
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
                event!(Level::ERROR, "Get list job by resume id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    async fn get_list_resume_by_job_id(&self, limit: Option<i32>, offset: i32, job_id: JobId)
                                       -> Result<Vec<MapResumeJob>, Error>
    {
        match sqlx::query("SELECT * FROM map_resume_job where job_id = $1 LIMIT $2 OFFSET $3 ")
            .bind(job_id.0)
            .bind(limit)
            .bind(offset)
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
                event!(Level::ERROR, "Get list job by job id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    //user
    async fn create_user(&self, new_user: AuthInfo)
                         -> Result<User, Error>
    {
        match sqlx::query("INSERT INTO users (email, password, company_id, role_id, is_delete) \
                            VALUES ($1, $2, $3, $4, $5) \
                            RETURNING id, email, password, company_id, role_id, is_delete")
            .bind(new_user.email)
            .bind(new_user.password)
            .bind(0)
            .bind(USER_ROLE_ID)
            .bind(false)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Create user for database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    async fn get_user_by_email(&self, user_email: String)
                               -> Result<User, Error>
    {
        match sqlx::query("SELECT * FROM USERS WHERE email = $1")
            .bind(user_email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password:row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Get user by email from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_user_by_id(&self, user_id: UserId)
                            -> Result<User, Error>
    {
        match sqlx::query("SELECT * FROM USERS WHERE id = $1")
            .bind(user_id.0)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Get user by id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_user(&self, limit: Option<i32>, offset: i32)
                           -> Result<Vec<User>, Error>
    {
        match sqlx::query("SELECT * FROM USERS LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(users) => Ok(users),
            Err(e) => {
                event!(Level::ERROR, "Get list user from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_user(&self, user_info: UserInfo)
                         -> Result<User, Error>
    {
        match sqlx::query(
            "Update users SET company_id = $1 \
                where email = $2 \
                RETURNING id, email, password, company_id, role_id, is_delete")
            .bind(user_info.company_id.0)
            .bind(user_info.email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Update user from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_password(&self, user: AuthInfo)
                             -> Result<User, Error>
    {
        match sqlx::query(
            "Update users SET password = $1 \
                where email = $2 \
                RETURNING id, email, password, company_id, role_id, is_delete")
            .bind(user.password)
            .bind(user.email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Update password for user from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn set_role(&self, user: UserInfo, role_id: RoleId)
                      -> Result<User, Error>
    {
        match sqlx::query(
            "Update users SET role_id = $1 \
                where id = $2 \
                RETURNING id, email, password, company_id, role_id, is_delete")
            .bind(role_id.0)
            .bind(user.id.0)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                event!(Level::ERROR, "Set role for user from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_user_by_id(&self, user_id: UserId)
                               -> Result<bool, Error>
    {
        match sqlx::query("Update users set is_delete = $1 where id = $2")
            .bind(true)
            .bind(user_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                event!(Level::ERROR, "Delete user by id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn create_role(&self, new_role: RoleInfo)
                         -> Result<Role, Error>
    {
        match sqlx::query("INSERT INTO roles (role, is_delete) \
                            VALUES ($1, $2)\
                            RETURNING id, role, is_delete")
            .bind(new_role.role)
            .bind(false)
            .map(|row: PgRow| Role {
                id: Some(RoleId(row.get("id"))),
                role: row.get("role"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(role) => Ok(role),
            Err(e) => {
                event!(Level::ERROR, "Create role for database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_role_by_id(&self, role_id: RoleId)
                            -> Result<Role, Error>
    {
        match sqlx::query("SELECT * FROM roles WHERE id = $1")
            .bind(role_id.0)
            .map(|row: PgRow| Role {
                id: Some(RoleId(row.get("id"))),
                role: row.get("role"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(role) => Ok(role),
            Err(e) => {
                event!(Level::ERROR, "Get role by id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_roles(&self)
                            -> Result<Vec<Role>, Error>
    {
        match sqlx::query("SELECT * FROM ROLES")
            .map(|row: PgRow| Role {
                id: Some(RoleId(row.get("id"))),
                role: row.get("role"),
                is_delete: row.get("is_delete")
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(roles) => Ok(roles),
            Err(e) => {
                event!(Level::ERROR, "Get list role from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_role(&self, role: Role)
                         -> Result<Role, Error>
    {
        match sqlx::query(
            "Update roles SET role = $1\
                            WHERE id = $2\
                            RETURNING id, role, is_delete")
            .bind(role.role)
            .bind(role.id.unwrap().0)
            .map(|row: PgRow| Role {
                id: Some(RoleId(row.get("id"))),
                role: row.get("role"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(role) => Ok(role),
            Err(e) => {
                event!(Level::ERROR, "Update role from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_role(&self, role_id: RoleId)
                         -> Result<bool, Error>
    {
        match sqlx::query("Update roles set is_delete = $1 where id = $2")
            .bind(true)
            .bind(role_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                event!(Level::ERROR, "Delete role from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn create_company(&self, new_company: NewCompany)
                            -> Result<Company, Error>
    {

        match sqlx::query("INSERT INTO companies (email, name, address, description, is_delete) \
                            VALUES ($1, $2, $3, $4, $5)\
                            RETURNING id, email, name, address, description, is_delete")
            .bind(new_company.email)
            .bind(new_company.name)
            .bind(new_company.address)
            .bind(new_company.description)
            .bind(false)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),
                is_delete: row.get("is_delete")

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(company) => Ok(company),
            Err(e) => {
                event!(Level::ERROR, "Create company for database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_company_by_email(&self, company_email: String)
                                  -> Result<Company, Error>
    {
        match sqlx::query("SELECT * FROM COMPANIES WHERE email = $1")
            .bind(company_email)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),
                is_delete: row.get("is_delete")

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(company) => Ok(company),
            Err(e) => {
                event!(Level::ERROR, "Get company by email from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_company_by_id(&self, company_id: CompanyId)
                               -> Result<Company, Error>
    {
        match sqlx::query("SELECT * FROM COMPANIES WHERE id = $1")
            .bind(company_id.0)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),
                is_delete: row.get("is_delete")

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(company) => Ok(company),
            Err(e) => {
                event!(Level::ERROR, "Get company by id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_company(&self, limit: Option<i32>, offset: i32)
                              -> Result<Vec<Company>, Error>
    {
        match sqlx::query("SELECT * FROM COMPANIES LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),
                is_delete: row.get("is_delete")

            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(companies) => Ok(companies),
            Err(e) => {
                event!(Level::ERROR, "Get list company from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_company(&self, company: Company)
                            -> Result<Company, Error>
    {
        match sqlx::query(
            "Update companies \
                            SET email = $1, name = $2, address = $3, description = $4 \
                            WHERE id = $5 \
                            RETURNING id, email, name, address, description, is_delete")
            .bind(company.email)
            .bind(company.name)
            .bind(company.address)
            .bind(company.description)
            .bind(company.id.unwrap().0)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),
                is_delete: row.get("is_delete")

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(company) => Ok(company),
            Err(e) => {
                event!(Level::ERROR, "Update company from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_company(&self, company_id: CompanyId)
                            -> Result<bool, Error>
    {
        match sqlx::query("Update companies set is_delete = $1 where id = $2")
            .bind(true)
            .bind(company_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                event!(Level::ERROR, "Delete company from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn create_job(&self, new_job: NewJob)
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
            Err(e) => {
                event!(Level::ERROR, "Create job for database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_job_by_id(&self, job_id: JobId)
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
                event!(Level::ERROR, "Create job from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }


    async fn get_list_job(&self, limit: Option<i32>, offset: i32)
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
                event!(Level::ERROR, "Get job by id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_job(&self, job: Job)
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
                event!(Level::ERROR, "Update job from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_job(&self, job_id: JobId)
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
                event!(Level::ERROR, "Delete job from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn create_resume(&self, new_resume: NewResume)
                           -> Result<Resume, Error>
    {
        match sqlx::query("INSERT INTO resumes (user_id, email, url, is_delete) \
                            VALUES ($1, $2, $3, $4)\
                            RETURNING id, user_id, email, url, is_delete")
            .bind(new_resume.user_id.0)
            .bind(new_resume.email)
            .bind(new_resume.url)
            .bind(false)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(e) => {
                event!(Level::ERROR, "Create resume for database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_resume_by_id(&self, resume_id: ResumeId)
                              -> Result<Resume, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES WHERE id = $1")
            .bind(resume_id.0)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(e) => {
                event!(Level::ERROR, "Get resume from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_resume_by_user_id(&self, limit: Option<i32>, offset: i32, user_id: UserId)
                                        -> Result<Vec<Resume>, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES WHERE user_id = $1 \
                                LIMIT $2 OFFSET $3 ")
            .bind(user_id.0)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(resumes) => Ok(resumes),
            Err(e) => {
                event!(Level::ERROR, "Get list resume by user id from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_resume(&self, resume: Resume)
                           -> Result<Resume, Error>
    {
        match sqlx::query(
            "Update resumes SET url = $1 \
                            WHERE id = $2 \
                            RETURNING id, user_id, email, url, is_delete")
            .bind(resume.url)
            .bind(resume.id.unwrap().0)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(e) => {
                event!(Level::ERROR, "Update resume from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_resume(&self, resume_id: ResumeId)
                           -> Result<bool, Error>
    {
        match sqlx::query("Update resumes set is_delete = $1 where id = $2")
            .bind(true)
            .bind(resume_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                event!(Level::ERROR, "Delete resume from database has error: {:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}