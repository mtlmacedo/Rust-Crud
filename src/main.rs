mod api;

use api::task::{
    get_task
};

use actix_web::{HtttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
fn main() -> std::io::Result<()> {   
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");    
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .service(get_task)
    })
    .bind(("127.0.0.1", 80))
    .run()
    .await
}
