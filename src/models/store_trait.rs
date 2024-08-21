use std::fmt;
use async_trait::async_trait;
use crate::service::handle_errors::Error;
use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::job::{Job, JobId, NewJob};
use crate::models::map_resume_job::{MapResumeJob, NewMapResumeJob};
use crate::models::resume::{NewResume, Resume, ResumeId};
use crate::models::role::{Role, RoleId, RoleInfo};
use crate::models::user::{AuthInfo, User, UserId, UserInfo};

#[async_trait]
pub trait StoreMethods: fmt::Debug + Send + Sync {
    // methods for map resume job
    async fn create_map_job_resume(&self, new_map_resume_job: NewMapResumeJob) -> Result<MapResumeJob, Error>;
    async fn get_list_job_by_resume(&self, resume_id: ResumeId) -> Result<Vec<MapResumeJob>, Error>;
    async fn get_list_resume_by_job_id(&self, limit: Option<i32>, offset: i32, job_id: JobId) -> Result<Vec<MapResumeJob>, Error>;

    //methods for users
    async fn create_user(&self, new_user: AuthInfo)
                         -> Result<User, Error>;
    async fn get_user_by_email(&self, user_email: String)
                               -> Result<User, Error>;

    async fn get_user_by_id(&self, user_id: UserId)
                            -> Result<User, Error>;
    async fn get_list_user(&self, limit: Option<i32>, offset: i32)
                           -> Result<Vec<User>, Error>;
    async fn update_user(&self, user_info: UserInfo)
                         -> Result<User, Error>;
    async fn delete_user_by_id(&self, user_id: UserId)
                               -> Result<bool, Error>;
    async fn update_password(&self, user: AuthInfo)
                             -> Result<User, Error>;
    async fn set_role(&self, user: UserInfo, role_id: RoleId)
                      -> Result<User, Error>;

    // methods for roles
    async fn create_role(&self, new_role: RoleInfo)
                         -> Result<Role, Error>;
    async fn get_role_by_id(&self, role_id: RoleId)
                            -> Result<Role, Error>;
    async fn get_list_roles(&self)
                            -> Result<Vec<Role>, Error>;
    async fn update_role(&self, role: Role)
                         -> Result<Role, Error>;
    async fn delete_role(&self, role_id: RoleId)
                         -> Result<bool, Error>;
    // methods for company
    async fn create_company(&self, new_company: NewCompany)
                            -> Result<Company, Error>;
    async fn get_company_by_email(&self, company_email: String)
                                  -> Result<Company, Error>;
    async fn get_company_by_id(&self, company_id: CompanyId)
                               -> Result<Company, Error>;
    async fn get_list_company(&self, limit: Option<i32>, offset: i32)
                              -> Result<Vec<Company>, Error>;
    async fn update_company(&self, company: Company)
                            -> Result<Company, Error>;
    async fn delete_company(&self, company_id: CompanyId)
                            -> Result<bool, Error>;

    async fn create_job(&self, new_job: NewJob)
                        -> Result<Job, Error>;
    async fn get_job_by_id(&self, job_id: JobId)
                           -> Result<Job, Error>;
    async fn get_list_job(&self, limit: Option<i32>, offset: i32)
                          -> Result<Vec<Job>, Error>;
    async fn update_job(&self, job: Job)
                        -> Result<Job, Error>;
    async fn delete_job(&self, job_id: JobId)
                        -> Result<bool, Error>;
    //methods for resume
    async fn create_resume(&self, new_resume: NewResume)
                           -> Result<Resume, Error>;
    async fn get_resume_by_id(&self, resume_id: ResumeId)
                              -> Result<Resume, Error>;
    async fn get_list_resume_by_user_id(&self, limit: Option<i32>, offset: i32, user_id: UserId)
                                        -> Result<Vec<Resume>, Error>;
    async fn update_resume(&self, resume: Resume)
                           -> Result<Resume, Error>;
    async fn delete_resume(&self, resume_id: ResumeId)
                           -> Result<bool, Error>;
}