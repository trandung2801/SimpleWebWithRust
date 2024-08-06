use crate::models::role::{RoleInfo, RoleId, Role};
use crate::models::store::Store;

use handle_errors::Error;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

pub trait RoleStoreMethods {
    async fn create_role(self, new_role: RoleInfo)
                         -> Result<Role, Error>;
    async fn get_role_by_id(self, role_id: RoleId)
                            -> Result<Role, Error>;
    async fn get_list_roles(self)
                            -> Result<Vec<Role>, Error>;
    async fn update_role(self, role_info: RoleInfo)
                         -> Result<Role, Error>;
    async fn delete_role(self, role_id: RoleId)
                         -> Result<bool, Error>;
}

impl RoleStoreMethods for Store {
    async fn create_role(self, new_role: RoleInfo)
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

    async fn get_role_by_id(self, role_id: RoleId)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_roles(self)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_role(self, role_info: RoleInfo)
                             -> Result<Role, Error>
    {
        match sqlx::query(
            "Update roles SET (role) \
                            VALUES ($1)\
                            RETURNING id, role, is_delete")
            .bind(role_info.role)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_role(self, role_id: RoleId)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}