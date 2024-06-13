use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use dotenv::dotenv;
use actix_web::{App, HttpServer, web};
use actix_files as fs;

use sea_orm::{ConnectOptions, Database, DbConn};
use tera::Tera;
use crate::adapters::auth::mem_session_processor::MemSessionProcessor;
use crate::application::common::session_processor::SessionProcessor;

use crate::ioc::IoC;
use crate::presentation::interactor_factory::InteractorFactory;


mod presentation;
mod application;
mod adapters;
mod domain;
mod ioc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();


    let workers = match std::env::var("WORKERS") {
        Ok(workers) => workers.parse::<usize>().ok(),
        Err(_) => None,
    };
    
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let postgres_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let db: Box<DbConn> = match {
        let mut opt = ConnectOptions::new(postgres_uri);
        opt.max_connections(40)
            .min_connections(5)
            .sqlx_logging(false);
        Database::connect(opt)
    }.await {
        Ok(db) => Box::new(db),
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        },
    };
    
    let app_builder = move || {
        
        let ioc_arc: Arc<dyn InteractorFactory> = Arc::new(IoC::new(
            db.clone(),
        ));
        let ioc_data: web::Data<dyn InteractorFactory> = web::Data::from(ioc_arc);
        
        let sp_arc: Arc<dyn SessionProcessor> = Arc::new(MemSessionProcessor::new());
        let sp_data: web::Data<dyn SessionProcessor> = web::Data::from(sp_arc);
        
        App::new()
            .service(
                fs::Files::new("/static", "./static").show_files_listing()
            )
            .service(web::scope("/api")
                .configure(presentation::web::rest::user::router)
                .configure(presentation::web::rest::session::router)
                .configure(presentation::web::rest::token::router)
                .configure(presentation::web::rest::payment::router)
            )
            .service(web::scope("")
                .configure(presentation::web::html::auth::router)
                .configure(presentation::web::html::dashboard::router)
                .configure(presentation::web::html::payments::router)
                .configure(presentation::web::html::tokens::router)
                .configure(presentation::web::html::docs::router)
                .configure(presentation::web::html::about::router)
            )
            .app_data(ioc_data)
            .app_data(sp_data)
            .app_data(web::Data::new(tera.clone()))
            .default_service(web::route().to(presentation::web::exception::not_found))
        // .wrap(Logger::new("[%s] [%{r}a] %U"))
    };


    let available_workers = workers.unwrap_or(
        match thread::available_parallelism() {
            Ok(parallelism) => usize::from(parallelism),
            Err(_) => 1,
        }
    );
    
    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(listener) => {
            log::info!("🚀 Server started at http://{}", listener.local_addr().unwrap());
            listener
        },
        Err(e) => {
            log::error!("Failed to bind to port {} in host {}: {}", port, host, e);
            std::process::exit(1);
        },
    };

    HttpServer::new(app_builder)
        .listen(listener)?
        .workers(available_workers)
        .run().await
}
