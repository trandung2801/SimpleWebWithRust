use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Company {
    pub id: Option<CompanyId>,
    pub name: String,
    pub email: String,
    pub address: String,
    pub description: String,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanyId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCompany {
    pub email: String,
    pub name: String,
    pub address: String,
    pub description: String,
}
