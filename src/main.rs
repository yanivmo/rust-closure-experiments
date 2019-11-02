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

fn create_config<'a>(conf: &'a Config) -> impl FnOnce(&mut web::ServiceConfig) + 'a {
    move |app: &mut web::ServiceConfig| {
        app.service(
            web::scope("/user")
                .route("", web::get().to(gen_get_user(conf)))
        );
    }
}

fn gen_get_user(conf: &Config) -> impl Fn() -> String {
    let my_own_conf_clone = conf.clone();
    move || get_user(&my_own_conf_clone)
}

fn get_user(conf: &Config) -> String {
    println!("Config {} is {} here!", conf.val3, conf.val1);
    "User McUser".to_string()
}
