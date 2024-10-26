#[warn(dead_code)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
#[derive(Debug)]
enum Status {
    Connected,
    Disconnected,
}

fn is_database_connect() -> Status {
    println!("Database is connected!");
    println!("Database other variable: {:?}", Status::Disconnected);
    return Status::Connected;
}

fn check_user_credentials(creds: Credentials) -> bool {
    if creds.username == String::from("axshivam") && creds.password == String::from("123") {
        return true;
    } else {
        return false;
    }
}

pub fn authenticate(creds: Credentials) {
    let database_status: Status = is_database_connect();
    check_user_credentials(creds);
    println!("Database status: {:?}", database_status);
}