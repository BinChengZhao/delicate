use crate::db::schema::casbin_rule;

#[derive(Queryable, Identifiable, Clone, Debug, Serialize, Deserialize, Default)]
#[table_name = "casbin_rule"]
pub(crate) struct CasbinRule {
    pub id: u64,
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

#[derive(Insertable, Clone, Debug, Serialize, Deserialize, Default)]
#[table_name = "casbin_rule"]
pub(crate) struct NewCasbinRule {
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RoleId{
   pub role_id:usize
}