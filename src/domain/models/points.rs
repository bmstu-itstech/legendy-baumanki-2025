use std::ops::{Add, Div};

use crate::domain::error::DomainError;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Points(i32);

impl Points {
    pub fn zero() -> Self {
        Points(0)
    }

    pub fn new(value: i32) -> Result<Points, DomainError> {
        if value < 0 {
            return Err(DomainError::InvalidValue(format!(
                "invalid Points: expected positive value, got {value}"
            )));
        }
        Ok(Self(value))
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
    
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }
    
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Add for Points {
    type Output = Points;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
