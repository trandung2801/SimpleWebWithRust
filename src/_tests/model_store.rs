use crate::config::config::Config;
use crate::models::job::JobId;
use crate::models::map_resume_job::{ NewMapResumeJob, MapResumeJobActions, MapResumeJobMac};
use crate::models::resume::ResumeId;
use crate::models::store::{Store, StoreActionBasic};
#[tokio::test]
async fn store_test() -> Result<(), handle_errors::Error>
{
    let config_env = Config::new().expect("Config env not set");

    let db_url = &format!(
        "postgres://{}:{}@{}:{}/{}",
        config_env.db_user,
        config_env.db_password,
        config_env.db_host,
        config_env.db_port,
        config_env.db_name
    );

    let store = <Store as StoreActionBasic>::new(&db_url).await;

    print!("Running create new map job resume ...");
    let new_map = NewMapResumeJob{
        resume_id: ResumeId(1),
        job_id: JobId(1),
    };
    match <MapResumeJobMac as MapResumeJobActions>::create(store.clone(), new_map).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running create new map job resume ...");
    match <MapResumeJobMac as MapResumeJobActions>::list_resume_by_job(store.clone(), Some(10), 0, JobId(1)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}