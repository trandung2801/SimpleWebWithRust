use crate::config::config::Config;
use crate::models::store::{Store, StoreActionBasic};
use crate::models::company::{CompanyMac, NewCompany, CompanyActions, CompanyId, Company};


#[tokio::test]
async fn company_test() -> Result<(), handle_errors::Error>
{
    let config_env = Config::new().expect("Config env not set");

    let db_url = &format!(
        "postgres://{}:{}@{}:{}/{}",
        config_env.postgres.db_user,
        config_env.postgres.db_password,
        config_env.postgres.db_host,
        config_env.postgres.db_port,
        config_env.postgres.db_name
    );
    let store = <Store as StoreActionBasic>::new(&db_url).await;

    print!("Running create new company ...");
    let new_company = NewCompany{
        email: "sotatek@gmail.com".to_string(),
        name: "Sotatek".to_string(),
        address: "2 Pham Van Bach".to_string(),
        description: "Company out source for blockchain".to_string()
    };
    match CompanyMac::create(store.clone(), new_company).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get company by email ...");
    let company_email = "sotatek@gmail.com".to_string();
    match CompanyMac::get_by_email(store.clone(), &company_email).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get company by id ...");
    match CompanyMac::get_by_id(store.clone(), CompanyId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get list company ...");
    match CompanyMac::list(store.clone(), Some(10), 0).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running update company ...");
    let company = Company {
        id: Some(CompanyId(2)),
        name: "Sotanext".to_string(),
        email: "sotanext@gmail.com".to_string(),
        address: "Tang 5 Golden Park So 2 Pham Van Bach".to_string(),
        description: "Company out source for blockchain".to_string(),
        is_delete: false
    };
    match CompanyMac::update(store.clone(), company).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running delete company ...");
    match CompanyMac::delete(store.clone(), CompanyId(2)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}