use crate::config::configEnv::ConfigEnv;
use crate::models::company::{CompanyId, CompanyMac};
use crate::models::job::JobId;
use crate::models::resume::{NewResume, Resume, ResumeMac, ResumeActions, ResumeId};
use crate::models::store::{Store, StoreActionBasic};
use crate::models::user::UserId;

#[tokio::test]
async fn resume_test() -> Result<(), handle_errors::Error>
{
    let config_env = ConfigEnv::new().expect("Config env not set");

    let db_url = &format!(
        "postgres://{}:{}@{}:{}/{}",
        config_env.db_user,
        config_env.db_password,
        config_env.db_host,
        config_env.db_port,
        config_env.db_name
    );
    let store = <Store as StoreActionBasic>::new(&db_url).await;

    print!("Running create new resume ...");
    let new_resume = NewResume{
        user_id: UserId(1),
        email: "123@gmail.com".to_string(),
        url: "dsadasdasdasdjaslkjda".to_string()
    };
    match ResumeMac::create(store.clone(), new_resume).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get resume by user id ...");
    match ResumeMac::get_by_user_id(store.clone(), UserId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get resume by id ...");
    match ResumeMac::get_by_id(store.clone(), ResumeId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get list resume by user id ...");
    match ResumeMac::list_by_user_id(store.clone(), Some(10), 1, UserId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get list resume by job id ...");
    match ResumeMac::list_by_job_id(store.clone(), Some(10), 1, JobId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running update resume ...");
    let resume = Resume {
        id: Some(ResumeId(1)),
        user_id: UserId(1),
        email: "123@gmail.com".to_string(),
        url: "dsadasdasdasdjaslkjda".to_string(),
        is_delete: false
    };
    match ResumeMac::update(store.clone(), resume).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running delete resume ...");
    match ResumeMac::delete(store.clone(), ResumeId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}