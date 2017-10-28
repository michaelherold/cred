use std::fmt;

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
    pub username: Option<String>,
    pub password: Option<String>,
}

impl fmt::Display for Credential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name != self.url {
            write!(f, "{} -- {}\n", self.name, self.url).unwrap();
        } else {
            write!(f, "{}\n", self.name).unwrap();
        }

        match self.username {
            Some(ref username) => {
                write!(f, "  Username: {}\n", username).unwrap();
            }
            None => {}
        };

        match self.password {
            Some(ref password) => write!(f, "  Password: {}\n", password),
            None => write!(f, ""),
        }
    }
}
