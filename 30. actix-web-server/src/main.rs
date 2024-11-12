use actix_web::{ get, web, App, HttpServer, Responder};

#[get("/home")]
async fn home() -> impl Responder {
    let response: String = String::from("Welcome to home page");
    return response;
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params: web::Path<(String, String)>) -> impl Responder {
    let response: String = format!("Hello {} {}", params.0, params.1);
    return response;
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(hello_user).service(home)
    }).bind(("127.0.0.1", 4000))?
    .run();

    println!("Server is running on port 4000");
    server.await
}
