use crate::domain::error::DomainError;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserID(i64);

impl UserID {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Username(String);

impl Username {
    pub fn new(s: String) -> Result<Self, DomainError> {
        if s == "" {
            return Err(DomainError::InvalidValue(
                "invalid Username: expected not empty string".to_string(),
            ));
        }
        Ok(Self(s))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullName(String);

impl FullName {
    pub fn new(s: String) -> Result<Self, DomainError> {
        if s == "" {
            return Err(DomainError::InvalidValue(
                "invalid FullName: expected not empty string".to_string(),
            ));
        }
        Ok(Self(s.into()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct GroupName(String);

impl GroupName {
    pub fn new(s: impl Into<String>) -> Result<Self, DomainError> {
        let group_regex: Regex =
            Regex::new(r"^[А-Я]{1,3}1?[0-9]?[КИЦ]?-[1-9][0-9]?(\.[1-9])?[0-9][АБМ]?$").unwrap();

        let s = s.into().to_uppercase();
        if !group_regex.is_match(&s) {
            return Err(DomainError::InvalidValue(
                format!("invalid GroupName: {0}", s).to_string(),
            ));
        }
        Ok(Self(s.into()))
    }

    pub fn is_first_course(&self) -> bool {
        let first_course_regex: Regex =
            Regex::new(r"^[А-Я]{1,3}1?[0-9]?[КИЦ]?-1(\.[1-9])?[0-9][АБМ]?$").unwrap();
        first_course_regex.is_match(&self.0)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: UserID,
    username: Option<Username>,
    full_name: FullName,
    group_name: GroupName,
}

impl User {
    pub fn new(
        id: UserID,
        username: Option<Username>,
        full_name: FullName,
        group_name: GroupName,
    ) -> Self {
        Self {
            id,
            username,
            full_name,
            group_name,
        }
    }

    pub fn id(&self) -> UserID {
        self.id
    }

    pub fn username(&self) -> &Option<Username> {
        &self.username
    }

    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    pub fn group_name(&self) -> &GroupName {
        &self.group_name
    }

    pub fn change_full_name(&mut self, new: FullName) {
        self.full_name = new;
    }

    pub fn change_group_name(&mut self, new: GroupName) {
        self.group_name = new;
    }
}
