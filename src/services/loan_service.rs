use crate::dtos::loan_dto::{CreateLoanDto, LoanResponseDto, UpdateLoanDto};
use crate::errors::loan_error::LoanError;
use crate::models::loan::Loan;
use crate::repositories::loan_repository::LoanRepository;
use chrono::Utc;
use std::sync::Arc;

pub struct LoanService {
    repository: Arc<dyn LoanRepository>,
}

impl LoanService {
    pub fn new(repository: Arc<dyn LoanRepository>) -> Self {
        Self { repository }
    }

    pub fn get_all_loans(&self) -> Result<Vec<LoanResponseDto>, LoanError> {
        let loans = self.repository.find_all()?;
        Ok(LoanResponseDto::from_loans(&loans))
    }

    pub fn get_loan_by_id(&self, id: i64) -> Result<LoanResponseDto, LoanError> {
        let loan = self.repository.find_by_id(id)?;
        Ok(LoanResponseDto::from_loan(&loan))
    }

    pub fn create_loan(&self, dto: CreateLoanDto) -> Result<LoanResponseDto, LoanError> {
        dto.validate_all().map_err(LoanError::validation)?;

        let due_date = dto.parse_due_date()
            .map_err(|e| LoanError::validation(e))??;

        if due_date <= Utc::now().date_naive() {
            return Err(LoanError::validation(
                "La fecha de vencimiento debe ser posterior a la fecha actual"
            ));
        }

        let loan = Loan::new(
            dto.amount,
            dto.interest_rate,
            due_date,
        );

        let created = self.repository.create(&loan)?;
        Ok(LoanResponseDto::from_loan(&created))
    }

    pub fn update_loan(&self, id: i64, dto: UpdateLoanDto) -> Result<LoanResponseDto, LoanError> {
        let existing = self.repository.find_by_id(id)?;

        let mut updated = existing.clone();

        if let Some(amount) = dto.amount {
            if amount <= 0.0 {
                return Err(LoanError::validation(
                    "El monto debe ser un número positivo"
                ));
            }
            updated.update_amount(amount)
                .map_err(LoanError::validation)?;
        }

        if let Some(interest_rate) = dto.interest_rate {
            if interest_rate < 0.0 {
                return Err(LoanError::validation(
                    "La tasa de interés no puede ser negativa"
                ));
            }
            updated.update_interest_rate(interest_rate)
                .map_err(LoanError::validation)?;
        }

        if let Some(due_date_str) = & dto.due_date {
            if let Ok(Some(new_due_date)) = dto.parse_due_date() {
                if new_due_date <= Utc::now().date_naive() {
                    return Err(LoanError::validation(
                        "La fecha de vencimiento debe ser posterior a la fecha actual"
                    ));
                }
                updated.update_due_date(new_due_date)
                    .map_err(LoanError::validation)?;
            }
        }

        if let Some(status) = dto.status {
            match status.as_str() {
                "approved" => updated.approve(),
                "rejected" => updated.reject(),
                "pending" => {}
                _ => return Err(LoanError::validation(
                    "Estado inválido. Debe ser: pending, approved o rejected"
                )),
            }
        }

        let result = self.repository.update(id, &updated)?;
        Ok(LoanResponseDto::from_loan(&result))
    }

    pub fn delete_loan(&self, id: i64) -> Result<(), LoanError> {
        self.repository.find_by_id(id)?;
        self.repository.delete(id)
    }

    pub fn approve_loan(&self, id: i64) -> Result<LoanResponseDto, LoanError> {
        let mut loan = self.repository.find_by_id(id)?;

        if !loan.is_pending() {
            return Err(LoanError::validation(
                "Solo se pueden aprobar préstamos en estado pendiente"
            ));
        }

        loan.approve();
        let updated = self.repository.update(id, &loan)?;
        Ok(LoanResponseDto::from_loan(&updated))
    }

    pub fn reject_loan(&self, id: i64) -> Result<LoanResponseDto, LoanError> {
        let mut loan = self.repository.find_by_id(id)?;

        if !loan.is_pending() {
            return Err(LoanError::validation(
                "Solo se pueden rechazar préstamos en estado pendiente"
            ));
        }

        loan.reject();
        let updated = self.repository.update(id, &loan)?;
        Ok(LoanResponseDto::from_loan(&updated))
    }

    pub fn get_pending_loans(&self) -> Result<Vec<LoanResponseDto>, LoanError> {
        let loans = self.repository.find_pending()?;
        Ok(LoanResponseDto::from_loans(&loans))
    }

    pub fn get_overdue_loans(&self) -> Result<Vec<LoanResponseDto>, LoanError> {
        let loans = self.repository.find_overdue()?;
        Ok(LoanResponseDto::from_loans(&loans))
    }

    pub fn calculate_total_amount(&self, id: i64) -> Result<f64, LoanError> {
        let loan = self.repository.find_by_id(id)?;
        Ok(loan.calculate_total_amount())
    }

    pub fn check_overdue(&self, id: i64) -> Result<bool, LoanError> {
        let loan = self.repository.find_by_id(id)?;
        Ok(loan.is_overdue())
    }
}