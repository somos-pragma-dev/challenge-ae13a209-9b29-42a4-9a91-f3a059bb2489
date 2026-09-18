use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use std::env;

mod controllers;
mod dtos;
mod errors;
mod models;
mod repositories;
mod services;

use controllers::loan_controller;
use errors::loan_error::AppError;
use services::loan_service::LoanService;

fn configure_app(cfg: &mut web::ServiceConfig) {
    let loan_service = web::Data::new(LoanService::new());
    
    cfg.app_data(loan_service)
        .service(
            web::scope("/api/v1")
                .service(
                    web::resource("/loans")
                        .route(web::get().to(loan_controller::get_all_loans))
                        .route(web::post().to(loan_controller::create_loan))
                )
                .service(
                    web::resource("/loans/{id}")
                        .route(web::get().to(loan_controller::get_loan_by_id))
                        .route(web::put().to(loan_controller::update_loan))
                        .route(web::delete().to(loan_controller::delete_loan))
                )
                .service(
                    web::resource("/loans/{id}/approve")
                        .route(web::post().to(loan_controller::approve_loan))
                )
                .service(
                    web::resource("/loans/{id}/reject")
                        .route(web::post().to(loan_controller::reject_loan))
                )
        )
        .route("/health", web::get().to(health_check));
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "loan-api",
        "version": "1.0.0"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("Iniciando servidor de API de préstamos...");
    
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("El puerto debe ser un número válido");
    
    info!("Servidor escuchando en {}:{}", host, port);
    
    HttpServer::new(configure_app)
        .bind(("0.0.0.0", port))?
        .run()
        .await
}