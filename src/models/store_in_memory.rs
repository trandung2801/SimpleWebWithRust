use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::job::{Job, JobId, NewJob};
use crate::models::map_resume_job::{MapResumeJob, MapResumeJobId, NewMapResumeJob};
use crate::models::resume::{NewResume, Resume, ResumeId};
use crate::models::role::{Role, RoleId, RoleInfo};
use crate::models::store_trait::StoreMethods;
use crate::models::user::{AuthInfo, User, UserId, UserInfo};
use crate::service::handle_errors::Error;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{event, Level};

#[derive(Clone, Debug)]
pub struct InMemoryStore {
    pub users: Arc<RwLock<HashMap<UserId, User>>>,
    pub roles: Arc<RwLock<HashMap<RoleId, Role>>>,
    pub companies: Arc<RwLock<HashMap<CompanyId, Company>>>,
    pub jobs: Arc<RwLock<HashMap<JobId, Job>>>,
    pub resumes: Arc<RwLock<HashMap<ResumeId, Resume>>>,
    pub map_resume_job: Arc<RwLock<HashMap<MapResumeJobId, MapResumeJob>>>,
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            users: Default::default(),
            roles: Default::default(),
            companies: Default::default(),
            jobs: Default::default(),
            resumes: Default::default(),
            map_resume_job: Default::default(),
        }
        // InMemoryStore {
        //     users: Default::default(),
        //     roles: Default::default(),
        //     companies: Default::default(),
        //     jobs: Default::default(),
        //     resumes: Default::default(),
        //     map_resume_job: Default::default()
        // }
    }
}

#[async_trait]
impl StoreMethods for InMemoryStore {
    // methods for map resume job
    async fn create_map_job_resume(
        &self,
        new_map_resume_job: NewMapResumeJob,
    ) -> Result<MapResumeJob, Error> {
        let len = self.map_resume_job.read().await.len() as i32;
        let id: i32 = if len == 0 { 1 } else { len + 1 };
        let map_resume_job = MapResumeJob {
            id: Some(MapResumeJobId(id)),
            resume_id: new_map_resume_job.resume_id,
            job_id: new_map_resume_job.job_id,
        };
        self.map_resume_job
            .write()
            .await
            .insert(map_resume_job.id.clone().unwrap(), map_resume_job.clone());
        Ok(map_resume_job)
    }

    async fn get_list_job_by_resume(
        &self,
        resume_id: ResumeId,
    ) -> Result<Vec<MapResumeJob>, Error> {
        Ok(self
            .map_resume_job
            .read()
            .await
            .iter()
            .filter_map(|(_k, v)| {
                if v.resume_id == resume_id {
                    Some(v)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>())
    }
    async fn get_list_resume_by_job_id(
        &self,
        limit: Option<i32>,
        offset: i32,
        job_id: JobId,
    ) -> Result<Vec<MapResumeJob>, Error> {
        Ok(self
            .map_resume_job
            .read()
            .await
            .iter()
            .filter_map(|(_k, v)| if v.job_id == job_id { Some(v) } else { None })
            .skip(offset as usize)
            .take((limit.unwrap() - offset) as usize)
            .cloned()
            .collect::<Vec<_>>())
    }
    //methods for users
    async fn create_user(&self, new_user: AuthInfo) -> Result<User, Error> {
        let len = self.users.read().await.len() as i32;
        let id: i32 = if len == 0 { 1 } else { len + 1 };
        let user = User {
            id: Some(UserId(id)),
            email: new_user.email,
            password: new_user.password,
            company_id: CompanyId(0),
            role_id: RoleId(2),
            is_delete: false,
        };

        self.users
            .write()
            .await
            .insert(user.id.clone().unwrap(), user.clone());
        Ok(user)
    }

    async fn get_user_by_email(&self, user_email: String) -> Result<User, Error> {
        let vec_user = self
            .users
            .read()
            .await
            .iter()
            .filter_map(|(_k, v)| if v.email == user_email { Some(v) } else { None })
            .cloned()
            .collect::<Vec<_>>();

        match vec_user.first().cloned() {
            Some(u) => Ok(u),
            None => {
                event!(Level::ERROR, "Get user by email in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, Error> {
        match self.users.write().await.get(&user_id).cloned() {
            Some(user) => Ok(user),
            None => {
                event!(Level::ERROR, "Get user by id in memory not found");
                return Err(Error::NotFound);
            }
        }
    }
    async fn get_list_user(&self, limit: Option<i32>, offset: i32) -> Result<Vec<User>, Error> {
        Ok(self
            .users
            .read()
            .await
            .values()
            .skip(offset as usize)
            .take((limit.unwrap() - offset) as usize)
            .cloned()
            .collect::<Vec<_>>())
    }
    async fn update_user(&self, user_info: UserInfo) -> Result<User, Error> {
        // Get user from user_info
        let user = self.get_user_by_id(user_info.id.clone()).await?;
        let user_update = User {
            id: Some(user_info.id.clone()),
            email: user_info.email,
            password: user.password,
            company_id: user_info.company_id,
            role_id: user_info.role_id,
            is_delete: user_info.is_delete,
        };
        match self
            .users
            .write()
            .await
            .get_mut(&user_update.id.clone().unwrap())
        {
            Some(u) => {
                *u = user_update.clone();
                Ok(user_update)
            }
            None => {
                event!(Level::ERROR, "User update in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn delete_user_by_id(&self, user_id: UserId) -> Result<bool, Error> {
        match self.users.write().await.remove(&user_id) {
            Some(_) => Ok(true),
            None => {
                event!(Level::ERROR, "User delete in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn update_password(&self, user: AuthInfo) -> Result<User, Error> {
        let _user = self.get_user_by_email(user.email).await?;
        let user_update = User {
            id: _user.id,
            email: _user.email,
            password: user.password,
            company_id: _user.company_id,
            role_id: _user.role_id,
            is_delete: _user.is_delete,
        };
        match self
            .users
            .write()
            .await
            .get_mut(&user_update.id.clone().unwrap())
        {
            Some(u) => {
                *u = user_update.clone();
                Ok(user_update)
            }
            None => {
                event!(Level::ERROR, "User update password in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn set_role(&self, user: UserInfo, role_id: RoleId) -> Result<User, Error> {
        let _user = self.get_user_by_email(user.email).await?;
        let user_update = User {
            id: _user.id,
            email: _user.email,
            password: _user.password,
            company_id: _user.company_id,
            role_id: role_id,
            is_delete: _user.is_delete,
        };
        match self
            .users
            .write()
            .await
            .get_mut(&user_update.id.clone().unwrap())
        {
            Some(u) => {
                *u = user_update.clone();
                Ok(user_update)
            }
            None => {
                event!(Level::ERROR, "User set role in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    // methods for role
    async fn create_role(&self, new_role: RoleInfo) -> Result<Role, Error> {
        let len = self.roles.read().await.len() as i32;
        let id: i32 = if len == 0 { 1 } else { len + 1 };
        let role = Role {
            id: Some(RoleId(id)),
            role: new_role.role,
            is_delete: false,
        };

        self.roles
            .write()
            .await
            .insert(role.id.clone().unwrap(), role.clone());
        Ok(role)
    }

    async fn get_role_by_id(&self, role_id: RoleId) -> Result<Role, Error> {
        match self.roles.write().await.get(&role_id).cloned() {
            Some(role) => Ok(role),
            None => {
                event!(Level::ERROR, "Get role by id in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn get_list_roles(&self) -> Result<Vec<Role>, Error> {
        Ok(self
            .roles
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>())
    }

    async fn update_role(&self, role: Role) -> Result<Role, Error> {
        match self.roles.write().await.get_mut(&role.id.clone().unwrap()) {
            Some(value) => {
                *value = role.clone();
                Ok(role)
            }
            None => {
                event!(Level::ERROR, "Role update in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn delete_role(&self, role_id: RoleId) -> Result<bool, Error> {
        match self.roles.write().await.remove(&role_id) {
            Some(_) => Ok(true),
            None => {
                event!(Level::ERROR, "Role delete in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    // methods for company
    async fn create_company(&self, new_company: NewCompany) -> Result<Company, Error> {
        let len = self.companies.read().await.len() as i32;
        let id: i32 = if len == 0 { 1 } else { len + 1 };
        let company = Company {
            id: Some(CompanyId(id)),
            name: new_company.name,
            email: new_company.email,
            address: new_company.address,
            description: new_company.description,
            is_delete: false,
        };
        self.companies
            .write()
            .await
            .insert(company.id.clone().unwrap(), company.clone());
        Ok(company)
    }

    async fn get_company_by_email(&self, company_email: String) -> Result<Company, Error> {
        let vec_company = self
            .companies
            .read()
            .await
            .iter()
            .filter_map(|(_k, v)| {
                if v.email == company_email {
                    Some(v)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();
        match vec_company.first().cloned() {
            Some(company) => Ok(company),
            None => {
                event!(Level::ERROR, "Get company by email in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn get_company_by_id(&self, company_id: CompanyId) -> Result<Company, Error> {
        match self.companies.write().await.get(&company_id).cloned() {
            Some(company) => Ok(company),
            None => {
                event!(Level::ERROR, "Get company by id in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn get_list_company(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Company>, Error> {
        Ok(self
            .companies
            .read()
            .await
            .values()
            .skip(offset as usize)
            .take((limit.unwrap() - offset) as usize)
            .cloned()
            .collect::<Vec<_>>())
    }

    async fn update_company(&self, company: Company) -> Result<Company, Error> {
        match self
            .companies
            .write()
            .await
            .get_mut(&company.id.clone().unwrap())
        {
            Some(value) => {
                *value = company.clone();
                Ok(company)
            }
            None => {
                event!(Level::ERROR, "Company update in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn delete_company(&self, company_id: CompanyId) -> Result<bool, Error> {
        match self.companies.write().await.remove(&company_id) {
            Some(_) => Ok(true),
            None => {
                event!(Level::ERROR, "Company delete in memory not found");
                return Err(Error::NotFound);
            }
        }
    }
    // methods for job
    async fn create_job(&self, new_job: NewJob) -> Result<Job, Error> {
        let len = self.jobs.read().await.len() as i32;
        let id: i32 = if len == 0 { 1 } else { len + 1 };
        let job = Job {
            id: Some(JobId(id)),
            job_name: new_job.job_name,
            company_id: new_job.company_id,
            location: new_job.location,
            quantity: new_job.quantity,
            salary: new_job.salary,
            job_level: new_job.job_level,
            description: new_job.description,
            is_delete: false,
        };
        self.jobs
            .write()
            .await
            .insert(job.id.clone().unwrap(), job.clone());
        Ok(job)
    }

    async fn get_job_by_id(&self, job_id: JobId) -> Result<Job, Error> {
        match self.jobs.write().await.get(&job_id).cloned() {
            Some(job) => Ok(job),
            None => {
                event!(Level::ERROR, "Get job by id in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn get_list_job(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Job>, Error> {
        Ok(self
            .jobs
            .read()
            .await
            .values()
            .skip(offset as usize)
            .take((limit.unwrap() - offset) as usize)
            .cloned()
            .collect::<Vec<_>>())
    }

    async fn update_job(&self, job: Job) -> Result<Job, Error> {
        match self.jobs.write().await.get_mut(&job.id.clone().unwrap()) {
            Some(value) => {
                *value = job.clone();
                Ok(job)
            }
            None => {
                event!(Level::ERROR, "Job update in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn delete_job(&self, job_id: JobId) -> Result<bool, Error> {
        match self.jobs.write().await.remove(&job_id) {
            Some(_) => Ok(true),
            None => {
                event!(Level::ERROR, "Job delete in memory not found");
                return Err(Error::NotFound);
            }
        }
    }
    //methods for resume
    async fn create_resume(&self, new_resume: NewResume) -> Result<Resume, Error> {
        let len = self.resumes.read().await.len() as i32;
        let id: i32 = if len == 0 { 1 } else { len + 1 };
        let resume = Resume {
            id: Some(ResumeId(id)),
            user_id: new_resume.user_id,
            email: new_resume.email,
            url: new_resume.url,
            is_delete: false,
        };
        self.resumes
            .write()
            .await
            .insert(resume.id.clone().unwrap(), resume.clone());
        Ok(resume)
    }

    async fn get_resume_by_id(&self, resume_id: ResumeId) -> Result<Resume, Error> {
        match self.resumes.write().await.get(&resume_id).cloned() {
            Some(resume) => Ok(resume),
            None => {
                event!(Level::ERROR, "Get resume by id in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn get_list_resume_by_user_id(
        &self,
        limit: Option<i32>,
        offset: i32,
        user_id: UserId,
    ) -> Result<Vec<Resume>, Error> {
        Ok(self
            .resumes
            .read()
            .await
            .iter()
            .filter_map(|(_k, v)| if v.user_id == user_id { Some(v) } else { None })
            .cloned()
            .skip(offset as usize)
            .take((limit.unwrap() - offset) as usize)
            .collect::<Vec<_>>())
    }

    async fn update_resume(&self, resume: Resume) -> Result<Resume, Error> {
        match self
            .resumes
            .write()
            .await
            .get_mut(&resume.id.clone().unwrap())
        {
            Some(value) => {
                *value = resume.clone();
                Ok(resume)
            }
            None => {
                event!(Level::ERROR, "Resume update in memory not found");
                return Err(Error::NotFound);
            }
        }
    }

    async fn delete_resume(&self, resume_id: ResumeId) -> Result<bool, Error> {
        match self.resumes.write().await.remove(&resume_id) {
            Some(_) => Ok(true),
            None => {
                event!(Level::ERROR, "Resume delete in memory not found");
                return Err(Error::NotFound);
            }
        }
    }
}
