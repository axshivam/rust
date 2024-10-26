#[warn(dead_code)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}


pub mod database_config {
    pub enum Status {
        Connected,
        Disconnected,
    }

    pub fn is_database_connect() -> Status {
        println!("Database is connected!");
        return Status::Connected;
    }
}

pub mod utils_functions {
    pub fn check_user_credentials(creds: super::Credentials) -> bool {
        if creds.username == String::from("axshivam") && creds.password == String::from("123") {
            return true;
        } else {
            return false;
        }
    }
}


pub fn authenticate(creds: Credentials) {
    let database_status: database_config::Status = database_config::is_database_connect();
    utils_functions::check_user_credentials(creds);
}