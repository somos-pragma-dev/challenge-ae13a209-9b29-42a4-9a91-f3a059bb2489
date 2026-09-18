use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLoanDto {
    #[validate(range(min = 0.01, message = "El monto debe ser mayor a cero")]
    pub amount: f64,
    #[validate(range(min = 0.001, message = "La tasa de interés debe ser mayor a cero")]
    pub interest_rate: f64,
    #[validate(custom = "validate_due_date")]
    pub due_date: String,
}

fn validate_due_date(date: &str) -> Result<(), validator::ValidationError> {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(naive_date) => {
            let today = chrono::Utc::now().date_naive();
            if naive_date <= today {
                let mut err = validator::ValidationError::new("due_date_must_be_future");
                err.add_param(serde_json::json!({"value": date}), &"due_date");
                err.message = Some("La fecha de vencimiento debe ser posterior a la fecha actual".into());
                Err(err)
            } else {
                Ok(())
            }
        }
        Err(_) => {
            let mut err = validator::ValidationError::new("invalid_date_format");
            err.add_param(serde_json::json!({"value": date}), &"due_date");
            err.message = Some("El formato de fecha debe ser YYYY-MM-DD".into());
            Err(err)
        }
    }
}

impl CreateLoanDto {
    pub fn parse_due_date(&self) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(&self.due_date, "%Y-%m-%d")
            .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    }

    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.amount <= 0.0 {
            errors.push("El monto debe ser mayor a cero".to_string());
        }

        if self.interest_rate <= 0.0 {
            errors.push("La tasa de interés debe ser mayor a cero".to_string());
        }

        if let Err(e) = self.parse_due_date() {
            errors.push(e);
        } else {
            let parsed_date = self.parse_due_date().unwrap();
            let today = chrono::Utc::now().date_naive();
            if parsed_date <= today {
                errors.push("La fecha de vencimiento debe ser posterior a la fecha actual".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateLoanDto {
    #[validate(range(min = 0.01, message = "El monto debe ser mayor a cero"))]
    pub amount: Option<f64>,
    #[validate(range(min = 0.001, message = "La tasa de interés debe ser mayor a cero"))]
    pub interest_rate: Option<f64>,
    #[validate(custom = "validate_due_date_optional")]
    pub due_date: Option<String>,
    pub status: Option<String>,
}

fn validate_due_date_optional(date: &str) -> Result<(), validator::ValidationError> {
    validate_due_date(date)
}

impl UpdateLoanDto {
    pub fn parse_due_date(&self) -> Result<Option<NaiveDate>, String> {
        match &self.due_date {
            Some(date_str) => {
                let parsed = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD".to_string())?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    pub fn validate_status(status: &Option<String>) -> Result<(), String> {
        match status {
            Some(s) => {
                let valid_statuses = ["pendiente", "aprobado", "rechazado"];
                if valid_statuses.contains(&s.to_lowercase().as_str()) {
                    Ok(())
                } else {
                    Err(format!(
                        "Estado inválido. Debe ser uno de: {}",
                        valid_statuses.join(", ")
                    ))
                }
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanResponseDto {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl LoanResponseDto {
    pub fn from_loan(loan: &crate::models::loan::Loan) -> Self {
        LoanResponseDto {
            id: loan.id,
            amount: loan.amount,
            interest_rate: loan.interest_rate,
            due_date: loan.due_date.format("%Y-%m-%d").to_string(),
            status: loan.status.clone(),
            created_at: loan.created_at.format("%Y-%m-%d").to_string(),
            updated_at: loan.updated_at.format("%Y-%m-%d").to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanListResponseDto {
    pub loans: Vec<LoanResponseDto>,
    pub total: usize,
}

impl LoanListResponseDto {
    pub fn from_loans(loans: &[crate::models::loan::Loan]) -> Self {
        LoanListResponseDto {
            loans: loans.iter().map(LoanResponseDto::from_loan).collect(),
            total: loans.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseDto {
    pub error: String,
    pub message: String,
    pub details: Option<Vec<String>>,
}

impl ErrorResponseDto {
    pub fn new(error: &str, message: &str) -> Self {
        ErrorResponseDto {
            error: error.to_string(),
            message: message.to_string(),
            details: None,
        }
    }

    pub fn with_details(error: &str, message: &str, details: Vec<String>) -> Self {
        ErrorResponseDto {
            error: error.to_string(),
            message: message.to_string(),
            details: Some(details),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponseDto<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

impl<T> SuccessResponseDto<T> {
    pub fn new(data: T) -> Self {
        SuccessResponseDto {
            success: true,
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        SuccessResponseDto {
            success: true,
            data,
            message: Some(message.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_loan_dto_validation() {
        let dto = CreateLoanDto {
            amount: 1000.0,
            interest_rate: 0.1,
            due_date: "2025-12-31".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_create_loan_dto_invalid_amount() {
        let dto = CreateLoanDto {
            amount: -100.0,
            interest_rate: 0.1,
            due_date: "2025-12-31".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_create_loan_dto_invalid_interest_rate() {
        let dto = CreateLoanDto {
            amount: 1000.0,
            interest_rate: -0.1,
            due_date: "2025-12-31".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_update_loan_dto_partial_update() {
        let dto = UpdateLoanDto {
            amount: Some(2000.0),
            interest_rate: None,
            due_date: None,
            status: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_loan_response_dto_from_loan() {
        let future_date = chrono::Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = crate::models::loan::Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        let response = LoanResponseDto::from_loan(&loan);
        assert_eq!(response.id, 1);
        assert_eq!(response.amount, 1000.0);
    }

    #[test]
    fn test_error_response_dto_creation() {
        let error = ErrorResponseDto::new("VALIDATION_ERROR", "Error de validación");
        assert_eq!(error.error, "VALIDATION_ERROR");
    }

    #[test]
    fn test_success_response_dto_creation() {
        let response: SuccessResponseDto<String> = SuccessResponseDto::new("OK".to_string());
        assert!(response.success);
    }
}