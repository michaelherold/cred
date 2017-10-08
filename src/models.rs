use super::schema::credentials;

#[derive(Queryable)]
pub struct Credential {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Insertable)]
#[table_name = "credentials"]
pub struct NewCredential<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
