use chrono::{NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanStatus {
    Pendiente,
    Aprobado,
    Rechazado,
}

impl fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanStatus::Pendiente => write!(f, "pendiente"),
            LoanStatus::Aprobado => write!(f, "aprobado"),
            LoanStatus::Rechazado => write!(f, "rechazado"),
        }
    }
}

impl std::convert::From<String> for LoanStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "aprobado" => LoanStatus::Aprobado,
            "rechazado" => LoanStatus::Rechazado,
            _ => LoanStatus::Pendiente,
        }
    }
}

impl std::convert::From<&String> for LoanStatus {
    fn from(s: &String) -> Self {
        LoanStatus::from(s.clone())
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    AsChangeset,
    Validate,
)]
#[diesel(table_name = loans)]
#[validate(schema(
    validate(loan.amount > 0, "amount must be positive"),
    validate(loan.interest_rate > 0.0, "interest_rate must be positive"),
    validate(loan.due_date > *&Utc::now().date_naive(), "due_date must be in the future")
))]
pub struct Loan {
    pub id: i64,
    #[validate(range(min = 0.01, message = "amount must be greater than zero")]
    pub amount: f64,
    #[validate(range(min = 0.001, message = "interest_rate must be greater than zero")]
    pub interest_rate: f64,
    pub due_date: NaiveDate,
    pub status: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl Loan {
    pub fn new(
        id: i64,
        amount: f64,
        interest_rate: f64,
        due_date: NaiveDate,
    ) -> Result<Self, String> {
        if amount <= 0.0 {
            return Err("El monto del préstamo debe ser mayor a cero".to_string());
        }
        if interest_rate <= 0.0 {
            return Err("La tasa de interés debe ser mayor a cero".to_string());
        }
        let today = Utc::now().date_naive();
        if due_date <= today {
            return Err("La fecha de vencimiento debe ser posterior a la fecha actual".to_string());
        }

        let now = Utc::now().date_naive();
        Ok(Loan {
            id,
            amount,
            interest_rate,
            due_date,
            status: LoanStatus::Pendiente.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn approve(&mut self) {
        self.status = LoanStatus::Aprobado.to_string();
        self.updated_at = Utc::now().date_naive();
    }

    pub fn reject(&mut self) {
        self.status = LoanStatus::Rechazado.to_string();
        self.updated_at = Utc::now().date_naive();
    }

    pub fn is_pending(&self) -> bool {
        self.status == LoanStatus::Pendiente.to_string()
    }

    pub fn is_approved(&self) -> bool {
        self.status == LoanStatus::Aprobado.to_string()
    }

    pub fn is_rejected(&self) -> bool {
        self.status == LoanStatus::Rechazado.to_string()
    }

    pub fn calculate_total_amount(&self) -> f64 {
        self.amount * (1.0 + self.interest_rate)
    }

    pub fn days_until_due(&self) -> i64 {
        let today = Utc::now().date_naive();
        (self.due_date - today).num_days()
    }

    pub fn is_overdue(&self) -> bool {
        self.days_until_due() < 0
    }

    pub fn update_amount(&mut self, new_amount: f64) -> Result<(), String> {
        if new_amount <= 0.0 {
            return Err("El nuevo monto debe ser mayor a cero".to_string());
        }
        self.amount = new_amount;
        self.updated_at = Utc::now().date_naive();
        Ok(())
    }

    pub fn update_interest_rate(&mut self, new_rate: f64) -> Result<(), String> {
        if new_rate <= 0.0 {
            return Err("La nueva tasa de interés debe ser mayor a cero".to_string());
        }
        self.interest_rate = new_rate;
        self.updated_at = Utc::now().date_naive();
        Ok(())
    }

    pub fn update_due_date(&mut self, new_due_date: NaiveDate) -> Result<(), String> {
        let today = Utc::now().date_naive();
        if new_due_date <= today {
            return Err("La nueva fecha de vencimiento debe ser posterior a la fecha actual".to_string());
        }
        self.due_date = new_due_date;
        self.updated_at = Utc::now().date_naive();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_loan_creation_with_valid_data() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, 1000.0, 0.1, future_date);
        assert!(loan.is_ok());
    }

    #[test]
    fn test_loan_creation_with_negative_amount() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, -100.0, 0.1, future_date);
        assert!(loan.is_err());
    }

    #[test]
    fn test_loan_creation_with_zero_interest_rate() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, 1000.0, 0.0, future_date);
        assert!(loan.is_err());
    }

    #[test]
    fn test_loan_creation_with_past_due_date() {
        let past_date = Utc::now().date_naive() - chrono::Duration::days(1);
        let loan = Loan::new(1, 1000.0, 0.1, past_date);
        assert!(loan.is_err());
    }

    #[test]
    fn test_loan_approve() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let mut loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        loan.approve();
        assert!(loan.is_approved());
    }

    #[test]
    fn test_loan_reject() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let mut loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        loan.reject();
        assert!(loan.is_rejected());
    }

    #[test]
    fn test_calculate_total_amount() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        assert_eq!(loan.calculate_total_amount(), 1100.0);
    }

    #[test]
    fn test_days_until_due() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(15);
        let loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        assert_eq!(loan.days_until_due(), 15);
    }

    #[test]
    fn test_is_overdue() {
        let past_date = Utc::now().date_naive() - chrono::Duration::days(1);
        let loan = Loan::new(1, 1000.0, 0.1, past_date).unwrap();
        assert!(loan.is_overdue());
    }
}