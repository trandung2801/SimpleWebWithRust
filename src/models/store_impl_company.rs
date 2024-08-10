use sqlx::{
    Row,
};
use sqlx::postgres::PgRow;
use handle_errors::Error;
use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::store::Store;

pub trait CompanyStoreMethod {
    async fn create_company(self, new_company: NewCompany)
                            -> Result<Company, Error>;
    async fn get_company_by_email(self, company_email: &String)
                                  -> Result<Company, Error>;
    async fn get_company_by_id(self, company_id: CompanyId)
                               -> Result<Company, Error>;
    async fn get_list_company(self, limit: Option<i32>, offset: i32)
                              -> Result<Vec<Company>, Error>;
    async fn update_company(self, company: Company)
                            -> Result<Company, Error>;
    async fn delete_company(self, company_id: CompanyId)
                                     -> Result<bool, Error>;
}

impl CompanyStoreMethod for Store {
    async fn create_company(self, new_company: NewCompany)
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

    async fn get_company_by_email(self, company_email: &String)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_company_by_id(self, company_id: CompanyId)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_company(self, limit: Option<i32>, offset: i32)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_company(self, company: Company)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_company(self, company_id: CompanyId)
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
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}