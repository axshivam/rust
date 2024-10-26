use auth_service::Credentials;
use auth_service::authenticate;

fn main() {
    let user: Credentials = Credentials {
        username: String::from("axshivam"),
        password: String::from("123"),
    };

    authenticate(user);
}
