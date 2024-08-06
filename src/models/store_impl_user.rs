use crate::models::company::CompanyId;
use crate::models::role::RoleId;
use crate::models::store::Store;
use crate::models::user::{AuthInfo, User, UserId, UserInfo};

use handle_errors::Error;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
pub trait UserStoreMethods {
    async fn create_user(self, new_user: AuthInfo)
                         -> Result<UserInfo, Error>;
    async fn get_user_by_email(self, user_email: &String)
    -> Result<User, Error>;

    async fn get_user_by_id(self, user_id: UserId)
                            -> Result<UserInfo, Error>;
    async fn get_list_user(self)
                           -> Result<Vec<UserInfo>, Error>;
    async fn update_user(self, user_info: UserInfo)
                         -> Result<UserInfo, Error>;
    async fn delete_user_by_email(self, user_email: String)
                                  -> Result<bool, Error>;
    async fn update_password(self, user: AuthInfo)
                             -> Result<UserInfo, Error>;
    async fn set_admin_role(self, user: UserInfo)
                            -> Result<UserInfo, Error>;
}

impl UserStoreMethods for Store {
    async fn create_user(self, new_user: AuthInfo)
                             -> Result<UserInfo, Error>
    {
        match sqlx::query("INSERT INTO users (email, password, is_delete) \
                            VALUES ($1, $2, $3)\
                            RETURNING id, email, company_id, role_id, is_delete")
            .bind(new_user.email)
            .bind(new_user.password)
            .bind(false)
            .map(|row: PgRow| UserInfo {
                id: UserId(row.get("id")),
                email:row.get("email"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
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
    async fn get_user_by_email(self, user_email: &String)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_user_by_id(self, user_id: UserId)
                                -> Result<UserInfo, Error>
    {
        match sqlx::query("SELECT * FROM USERS WHERE id = $1")
            .bind(user_id.0)
            .map(|row: PgRow| UserInfo {
                id: UserId(row.get("id")),
                email:row.get("email"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
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

    async fn get_list_user(self)
                               -> Result<Vec<UserInfo>, Error>
    {
        match sqlx::query("SELECT * FROM USERS")
            .map(|row: PgRow| UserInfo {
                id: UserId(row.get("id")),
                email:row.get("email"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
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

    async fn update_user(self, user_info: UserInfo)
                             -> Result<UserInfo, Error>
    {
        match sqlx::query(
            "Update users SET (company_id) \
                values($1) \
                where email = $2 \
                RETURNING id, email, company_id, role_id, is_delete")
            .bind(user_info.company_id.0)
            .bind(user_info.email)
            .map(|row: PgRow| UserInfo {
                id: UserId(row.get("id")),
                email:row.get("email"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
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

    async fn update_password(self, user: AuthInfo)
                                 -> Result<UserInfo, Error>
    {
        match sqlx::query(
            "Update users SET password = $1 \
                where email = $2 \
                RETURNING id, email, company_id, role_id, is_delete")
            .bind(user.password)
            .bind(user.email)
            .map(|row: PgRow| UserInfo {
                id: UserId(row.get("id")),
                email:row.get("email"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
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

    async fn set_admin_role(self, user: UserInfo)
                                -> Result<UserInfo, Error>
    {
        match sqlx::query(
            "Update users SET role_id = $1 \
                where email = $2 \
                RETURNING id, email, company_id, role_id, is_delete")
            .bind(user.role_id.0)
            .bind(user.email)
            .map(|row: PgRow| UserInfo {
                id: UserId(row.get("id")),
                email:row.get("email"),
                company_id: CompanyId(row.get("company_id")),
                role_id: RoleId(row.get("role_id")),
                is_delete: row.get("is_delete")
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

    async fn delete_user_by_email(self, user_email: String)
                                  -> Result<bool, Error>
    {
        match sqlx::query("Update users set is_delete = $1 where email = $2")
            .bind(true)
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
}