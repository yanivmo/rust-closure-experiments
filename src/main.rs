use actix_web::{web, App, HttpServer};

#[derive(Clone)]
struct Config {
    val1: String,
    val2: String,
    val3: String,
}

fn main() {
    let conf = Config {
        val1: "just".to_string(),
        val2: "some".to_string(),
        val3: "data".to_string(),
    };

    HttpServer::new(move ||
        App::new().configure(create_config(&conf))
    )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

fn create_config(_conf: &Config) -> impl FnOnce(&mut web::ServiceConfig) {
    move |app: &mut web::ServiceConfig| {
        app.service(
            web::scope("/user")
                .route("", web::get().to(get_user))
        );
    }
}

fn get_user() -> String {
    "User McUser".to_string()
}
