use crates_and_packages::auth_utils::models::Credentials;
use crates_and_packages::credentials;

// use crates_and_packages::{Credentials, credentials};

fn main() {
    let cred: Credentials = Credentials {
        username: String::from("axshivam"),
        password: String::from("admin@123"),
    };

    credentials(cred);
    // crates_and_packages::credentials(cred);
}
