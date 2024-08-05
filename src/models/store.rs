use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use tracing::log::SetLoggerError;
use handle_errors::Error;
use crate::models::user::{AuthInfo, User, UserId, UserInfo};
use crate::models::company::{Company, CompanyInfo, CompanyId};
use crate::models::resume::{Resume, ResumeId, ResumeInfo};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
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
//impl for user store
impl Store {
    pub async fn create_user(self, new_user: AuthInfo)
                             -> Result<User, Error>
    {
        match sqlx::query("INSERT INTO users (email, password, company, is_admin ) \
                            VALUES ($1, $2, $3, $4)\
                            RETURNING id, email, password, company, is_admin")
            .bind(new_user.email)
            .bind(new_user.password)
            .bind("".to_string())
            .bind(false)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company: row.get("company"),
                is_admin: row.get("is_admin"),

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
    pub async fn get_user_by_email(self, user_email: &String)
                                   -> Result<User, Error>
    {
        match sqlx::query("SELECT * FROM USERS WHERE email = $1")
            .bind(user_email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company: row.get("company"),
                is_admin: row.get("is_admin"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn get_list_user(self)
        -> Result<Vec<User>, Error>
    {
        match sqlx::query("SELECT * FROM USERS")
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company: row.get("company"),
                is_admin: row.get("is_admin"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(users) => Ok(users),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn update_user(self, user_info: UserInfo)
                             -> Result<User, Error>
    {
        match sqlx::query(
            "Update users SET company = $1 \
        where email = $2 \
        RETURNING id, email, password, company, is_admin")
            .bind(user_info.company)
            .bind(user_info.email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company: row.get("company"),
                is_admin: row.get("is_admin"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn delete_user_by_email(self, user_email: String)
                                      -> Result<bool, Error>
    {
        match sqlx::query("Delete from users where email = $1")
            .bind(user_email)
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

    pub async fn update_password(self, user: AuthInfo)
                                 -> Result<User, Error>
    {
        match sqlx::query(
            "Update users SET password = $1 \
        where email = $2 \
        RETURNING id, email, password, company, is_admin")
            .bind(user.password)
            .bind(user.email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company: row.get("company"),
                is_admin: row.get("is_admin"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn set_admin_role(self, user: UserInfo)
                                -> Result<User, Error>
    {
        match sqlx::query(
            "Update users SET is_admin = $1 \
        where email = $2 \
        RETURNING id, email, password, company, is_admin")
            .bind(true)
            .bind(user.email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email:row.get("email"),
                password: row.get("password"),
                company: row.get("company"),
                is_admin: row.get("is_admin"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}
//impl for company
impl Store {
    pub async fn create_company(self, new_company: CompanyInfo)
                                -> Result<Company, Error>
    {
        match sqlx::query("INSERT INTO companies (email, name, address, description) \
                            VALUES ($1, $2, $3, $4)\
                            RETURNING id, email, name, address, description")
            .bind(new_company.email)
            .bind(new_company.name)
            .bind(new_company.address)
            .bind(new_company.description)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(company) => Ok(company),
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

    pub async fn get_company_by_email(self, company_email: &String)
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

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn get_list_company(self)
                                  -> Result<Vec<Company>, Error>
    {
        match sqlx::query("SELECT * FROM COMPANIES")
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),

            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(users) => Ok(users),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn update_company(self, company_info: CompanyInfo)
                                -> Result<Company, Error>
    {
        match sqlx::query(
            "Update companies SET (email, name, address, description ) \
                            VALUES ($1, $2, $3, $4)\
                            RETURNING id, email, address, description")
            .bind(company_info.email)
            .bind(company_info.name)
            .bind(company_info.address)
            .bind(company_info.description)
            .map(|row: PgRow| Company {
                id: Some(CompanyId(row.get("id"))),
                email:row.get("email"),
                name: row.get("name"),
                address: row.get("address"),
                description: row.get("description"),

            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn delete_company_by_email(self, company_info: CompanyInfo)
                                         -> Result<bool, Error>
    {
        match sqlx::query("Delete from companies where email = $1")
            .bind(company_info.email)
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

//impl for resume
impl Store {
    pub async fn create_resume(self, new_resume: ResumeInfo)
                                -> Result<Resume, Error>
    {
        match sqlx::query("INSERT INTO resumes (user_id, email, url, status) \
                            VALUES ($1, $2, $3, $4)\
                            RETURNING id, user_id, email, url, status")
            .bind(new_resume.user_id)
            .bind(new_resume.email)
            .bind(new_resume.url)
            .bind(new_resume.status)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                email:row.get("email"),
                user_id:row.get("user_id"),
                url: row.get("url"),
                status: row.get("status"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(company) => Ok(company),
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
}