use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use log::{error, info};

use crate::dtos::loan_dto::{
    CreateLoanDto, ErrorResponseDto, LoanResponseDto, SuccessResponseDto, UpdateLoanDto,
};
use crate::errors::loan_error::AppError;
use crate::models::loan::Loan;
use crate::services::loan_service::LoanService;

pub async fn get_all_loans(
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    info!("Obteniendo todos los préstamos");
    
    match service.get_all().await {
        Ok(loans) => {
            let response = LoanResponseDto::from_loans(&loans);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamos obtenidos exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al obtener préstamos: {}", e);
            Err(AppError::NotFound("No se encontraron préstamos".to_string()))
        }
    }
}

pub async fn get_loan_by_id(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Obteniendo préstamo con ID: {}", loan_id);
    
    match service.get_by_id(loan_id).await {
        Ok(Some(loan)) => {
            let response = LoanResponseDto::from_loan(&loan);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo obtenido exitosamente",
            )))
        }
        Ok(None) => {
            error!("Préstamo no encontrado: {}", loan_id);
            Err(AppError::NotFound(format!(
                "Préstamo con ID {} no encontrado",
                loan_id
            )))
        }
        Err(e) => {
            error!("Error al obtener préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al procesar la solicitud".to_string(),
            ))
        }
    }
}

pub async fn create_loan(
    dto: web::Json<CreateLoanDto>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    info!("Creando nuevo préstamo");
    
    if let Err(errors) = dto.validate_all() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::with_details(
            "VALIDATION_ERROR",
            "Error de validación",
            errors,
        )));
    }
    
    let due_date = dto.parse_due_date()?;
    
    let mut loan = Loan::new(
        dto.amount,
        dto.interest_rate,
        due_date,
    );
    
    match service.create(&mut loan).await {
        Ok(created) => {
            info!("Préstamo creado exitosamente con ID: {}", created.id);
            let response = LoanResponseDto::from_loan(&created);
            Ok(HttpResponse::Created().json(SuccessResponseDto::with_message(
                response,
                "Préstamo creado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al crear préstamo: {}", e);
            Err(AppError::InternalServerError(
                "Error al crear el préstamo".to_string(),
            ))
        }
    }
}

pub async fn update_loan(
    id: web::Path<i64>,
    dto: web::Json<UpdateLoanDto>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Actualizando préstamo con ID: {}", loan_id);
    
    if let Some(ref amount) = dto.amount {
        if *amount <= 0.0 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
                "INVALID_AMOUNT",
                "El monto debe ser mayor a 0",
            )));
        }
    }
    
    if let Some(ref interest_rate) = dto.interest_rate {
        if *interest_rate < 0.0 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
                "INVALID_INTEREST_RATE",
                "La tasa de interés no puede ser negativa",
            )));
        }
    }
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    let mut loan = existing.unwrap();
    
    if let Some(amount) = dto.amount {
        loan.update_amount(amount)?;
    }
    
    if let Some(interest_rate) = dto.interest_rate {
        loan.update_interest_rate(interest_rate)?;
    }
    
    if let Some(ref due_date_str) = dto.due_date {
        let new_due_date = chrono::NaiveDate::parse_from_str(
            due_date_str,
            "%Y-%m-%d",
        )
        .map_err(|_| AppError::BadRequest("Formato de fecha inválido. Use YYYY-MM-DD".to_string()))?;
        
        if new_due_date <= chrono::Utc::now().date_naive() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
                "INVALID_DUE_DATE",
                "La fecha de vencimiento debe ser posterior a hoy",
            )));
        }
        
        loan.update_due_date(new_due_date)?;
    }
    
    match service.update(&loan).await {
        Ok(updated) => {
            info!("Préstamo {} actualizado exitosamente", loan_id);
            let response = LoanResponseDto::from_loan(&updated);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo actualizado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al actualizar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al actualizar el préstamo".to_string(),
            ))
        }
    }
}

pub async fn delete_loan(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Eliminando préstamo con ID: {}", loan_id);
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    match service.delete(loan_id).await {
        Ok(_) => {
            info!("Préstamo {} eliminado exitosamente", loan_id);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => {
            error!("Error al eliminar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al eliminar el préstamo".to_string(),
            ))
        }
    }
}

pub async fn approve_loan(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Aprobando préstamo con ID: {}", loan_id);
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    let mut loan = existing.unwrap();
    
    if loan.is_approved() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_APPROVED",
            "El préstamo ya está aprobado",
        )));
    }
    
    if loan.is_rejected() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_REJECTED",
            "No se puede aprobar un préstamo rechazado",
        )));
    }
    
    loan.approve();
    
    match service.update(&loan).await {
        Ok(updated) => {
            info!("Préstamo {} aprobado exitosamente", loan_id);
            let response = LoanResponseDto::from_loan(&updated);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo aprobado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al aprobar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al aprobar el préstamo".to_string(),
            ))
        }
    }
}

pub async fn reject_loan(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Rechazando préstamo con ID: {}", loan_id);
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    let mut loan = existing.unwrap();
    
    if loan.is_rejected() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_REJECTED",
            "El préstamo ya está rechazado",
        )));
    }
    
    if loan.is_approved() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_APPROVED",
            "No se puede rechazar un préstamos aprobado",
        )));
    }
    
    loan.reject();
    
    match service.update(&loan).await {
        Ok(updated) => {
            info!("Préstamo {} rechazado exitosamente", loan_id);
            let response = LoanResponseDto::from_loan(&updated);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo rechazado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al rechazar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al rechazar el préstamo".to_string(),
            ))
        }
    }
}