use crate::config::config::Config;
use crate::models::company::{CompanyId, CompanyMac};
use crate::models::job::JobId;
use crate::models::resume::{NewResume, Resume, ResumeMac, ResumeActions, ResumeId};
use crate::models::store::{Store, StoreMethods};
use crate::models::user::UserId;

#[tokio::test]
async fn resume_test() -> Result<(), handle_errors::Error>
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

    print!("Running create new resume ...");
    let new_resume = NewResume{
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
        url: "dsadasdasdasdjaslkjda".to_string()
    };
    match ResumeMac::create(store.clone(), new_resume).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get resume by user id ...");
    match ResumeMac::get_by_user_id(store.clone(), UserId(7)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get resume by id ...");
    match ResumeMac::get_by_id(store.clone(), ResumeId(2)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get list resume by user id ...");
    match ResumeMac::list_by_user_id(store.clone(), Some(10), 1, UserId(7)).await {
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
        id: Some(ResumeId(2)),
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
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
    match ResumeMac::delete(store.clone(), ResumeId(2)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}