use crate::config::config::Config;
use crate::models::store::{Store, StoreMethods};
use crate::models::company::CompanyId;
use crate::models::role::{ADMIN_ROLE_ID, HR_ROLE_ID, RoleId, USER_ROLE_ID};
use crate::models::user::{AuthInfo, UserActions, UserId, UserMac, UserInfo};
#[tokio::test]
async fn user_test() -> Result<(), handle_errors::Error>
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

    print!("Running get user by email ...");
    let email = "user1@gmail.com".to_string();
    match UserMac::get_by_email(store.clone(), &email).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get user by id ...");
    match UserMac::get_by_id(store.clone(), UserId(3)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running get list user ...");
    match UserMac::list(store.clone(),  Some(10), 0).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running update user ...");
    let user_update = UserInfo {
        id: UserId(8),
        email: "user3@gmail.com".to_string(),
        company_id: CompanyId(1),
        role_id: RoleId(USER_ROLE_ID),
        is_delete: false,
    };
    match UserMac::update_user(store.clone(),  user_update).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running update password user ...");
    let update_password = AuthInfo {
        email: "user3@gmail.com".to_string(),
        password: "123456789".to_string()
    };
    match UserMac::update_password(store.clone(),  update_password).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running set hr role for user ...");
    let user_set_hr = UserInfo {
        id: UserId(8),
        email: "user3@gmail.com".to_string(),
        company_id: CompanyId(1),
        role_id: RoleId(USER_ROLE_ID),
        is_delete: false,
    };
    match UserMac::set_role(store.clone(),  user_set_hr, RoleId(HR_ROLE_ID)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running set admin role for user ...");
    let user_set_hr = UserInfo {
        id: UserId(8),
        email: "user3@gmail.com".to_string(),
        company_id: CompanyId(1),
        role_id: RoleId(HR_ROLE_ID),
        is_delete: false,
    };
    match UserMac::set_role(store.clone(),  user_set_hr, RoleId(ADMIN_ROLE_ID)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    print!("Running delete user ...");
    match UserMac::delete(store.clone(), UserId(8)).await {
        Ok(_) => println!("✓"),
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}