use crate::errors::loan_error::LoanError;
use crate::models::loan::Loan;
use diesel::prelude::*;
use diesel::PgConnection;
use std::sync::Arc;

pub trait LoanRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<Loan>, LoanError>;
    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError>;
    fn create(&self, loan: &Loan) -> Result<Loan, LoanError>;
    fn update(&self, id: i64, loan: &Loan) -> Result<Loan, LoanError>;
    fn delete(&self, id: i64) -> Result<(), LoanError>;
    fn find_pending(&self) -> Result<Vec<Loan>, LoanError>;
    fn find_overdue(&self) -> Result<Vec<Loan>, LoanError>;
}

pub struct DieselLoanRepository {
    connection: Arc<PgConnection>,
}

impl DieselLoanRepository {
    pub fn new(connection: Arc<PgConnection>) -> Self {
        Self { connection }
    }
}

impl LoanRepository for DieselLoanRepository {
    fn find_all(&self) -> Result<Vec<Loan>, LoanError> {
        use crate::schema::loans::dsl::*;

        let result = loans
            .load::<Loan>(&*self.connection)
            .map_err(LoanError::from)?;

        Ok(result)
    }

    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError> {
        use crate::schema::loans::dsl::*;

        loans
            .filter(id.eq(id))
            .first::<Loan>(&*self.connection)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => LoanError::not_found(&id.to_string()),
                _ => LoanError::from(e),
            })
    }

    fn create(&self, loan: &Loan) -> Result<Loan, LoanError> {
        use crate::schema::loans;

        diesel::insert_into(loans::table)
            .values(loan)
            .get_result(&*self.connection)
            .map_err(LoanError::from)
    }

    fn update(&self, id: i64, loan: &Loan) -> Result<Loan, LoanError> {
        use crate::schema::loans::dsl::*;

        let rows_updated = diesel::update(loans.filter(id.eq(id)))
            .set(loan)
            .execute(&*self.connection)
            .map_err(LoanError::from)?;

        if rows_updated == 0 {
            return Err(LoanError::not_found(&id.to_string()));
        }

        self.find_by_id(id)
    }

    fn delete(&self, id: i64) -> Result<(), LoanError> {
        use crate::schema::loans::dsl::*;

        let rows_deleted = diesel::delete(loans.filter(id.eq(id)))
            .execute(&*self.connection)
            .map_err(LoanError::from)?;

        if rows_deleted == 0 {
            return Err(LoanError::not_found(&id.to_string()));
        }

        Ok(())
    }

    fn find_pending(&self) -> Result<Vec<Loan>, LoanError> {
        use crate::schema::loans::dsl::*;

        loans
            .filter(status.eq("pending"))
            .load::<Loan>(&*self.connection)
            .map_err(LoanError::from)
    }

    fn find_overdue(&self) -> Result<Vec<Loan>, LoanError> {
        use crate::models::loan::LoanStatus;
        use crate::schema::loans::dsl::*;
        use chrono::Utc;

        let today = Utc::now().date_naive();

        let result = loans
            .filter(status.eq(LoanStatus::Approved.to_string()))
            .filter(due_date.lt(today))
            .load::<Loan>(&*self.connection)
            .map_err(LoanError::from)?;

        Ok(result)
    }
}