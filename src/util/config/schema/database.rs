
use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
pub struct ConfigDatabase {
    host: String,
    db_name: String,
    password: String,
    username: String,
}

impl ConfigDatabase {
    pub fn get_db_url(&self) -> String {
        format!("postgres://{username}:{password}@{host}/{db_name}",
            username=self.username,
            password=self.password,
            host=self.host,
            db_name=self.db_name
        )
    }
    pub fn get_db_url_censored(&self) -> String {
        format!("postgres://{username}:****@{host}/{db_name}",
            username=self.username,
            host=self.host,
            db_name=self.db_name
        )
    }
}