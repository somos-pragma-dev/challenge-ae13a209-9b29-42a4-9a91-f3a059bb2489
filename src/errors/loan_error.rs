use actix_web::{http::StatusCode, ResponseError};
use std::fmt;

#[derive(Debug) Clone)]
pub enum LoanError {
    NotFound(String),
    Validation(String),
    Repository(String),
    Service(String),
}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanError::NotFound(msg) => write!(f, "Préstamo no encontrado: {}", msg),
            LoanError::Validation(msg) => write!(f, "Error de validación: {}", msg),
            LoanError::Repository(msg) => write!(f, "Error de base de datos: {}", msg),
            LoanError::Service(msg) => write!(f, "Error en el servicio: {}", msg),
        }
    }
}

impl std::error::Error for LoanError {}

impl ResponseError for LoanError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoanError::NotFound(_) => StatusCode::NOT_FOUND,
            LoanError::Validation(_) => StatusCode::BAD_REQUEST,
            LoanError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoanError::Service(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        actix_web::HttpResponse::build(status)
            .json(serde_json::json!({
                "error": self.error_type(),
                "message": message
            }))
    }
}

impl LoanError {
    fn error_type(&self) -> &'static str {
        match self {
            LoanError::NotFound(_) => "NOT_FOUND",
            LoanError::Validation(_) => "VALIDATION_ERROR",
            LoanError::Repository(_) => "REPOSITORY_ERROR",
            LoanError::Service(_) => "SERVICE_ERROR",
        }
    }

    pub fn not_found(id: &str) -> Self {
        LoanError::NotFound(format!("No existe un préstamo con ID: {}", id))
    }

    pub fn validation(message: impl Into<String>) -> Self {
        LoanError::Validation(message.into())
    }

    pub fn repository(message: impl Into<String>) -> Self {
        LoanError::Repository(message.into())
    }

    pub fn service(message: impl Into<String>) -> Self {
        LoanError::Service(message.into())
    }
}

impl From<diesel::result::Error> for LoanError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                LoanError::Repository("Registro no encontrado en la base de datos".to_string())
            }
            _ => LoanError::Repository(format!("Error de base de datos: {}", err)),
        }
    }
}

impl From<diesel::dsl::EqAllErrors> for LoanError {
    fn from(_err: diesel::dsl::EqAllErrors) -> Self {
        LoanError::Repository("Error al construir consulta".to_string())
    }
}

impl serde::Serialize for LoanError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}