use crate::config::config::Config;
use crate::models::company::CompanyId;
use crate::models::job::{JobMac, NewJob, JobActions, JobId, Job};
use crate::models::store::{Store, StoreMethods};

#[tokio::test]
async fn job_test() -> Result<(), handle_errors::Error>
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
    let store = Store::new(&db_url).await;

    println!("Running create new job ... ");
    let new_job = NewJob {
        job_name: "Intern Web3".to_string(),
        company_id: CompanyId(1),
        location: "So 2 Pham Van Bach".to_string(),
        quantity: 1,
        salary: 1_000_000,
        job_level: "Intern".to_string(),
        description: "Intern from UET has knowledge rust".to_string(),
    };

    match JobMac::create(store.clone(), new_job).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    println!("Running get job by id ... ");
    match JobMac::get_by_id(store.clone(), JobId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    println!("Running get list job... ");
    match JobMac::list(store.clone(), Some(10), 1).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    println!("Running update job ... ");
    let job = Job{
        id: Some(JobId(1)),
        job_name: "Intern Web3".to_string(),
        company_id: CompanyId(1),
        location: "So 2 Pham Van Bach".to_string(),
        quantity: 1,
        salary: 1_000_000,
        job_level: "Intern".to_string(),
        description: "Intern from UET has knowledge rust".to_string(),
        is_delete: false,
    };

    match JobMac::update(store.clone(), job).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    println!("Running delete job... ");
    match JobMac::delete(store.clone(), JobId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}