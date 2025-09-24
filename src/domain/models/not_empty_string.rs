#[macro_export]
macro_rules! not_empty_string_impl {
    ($class:ident) => {
        impl $class {
            pub fn new(s: String) -> Result<Self, DomainError> {
                if s == "" {
                    Err(DomainError::InvalidValue(format!(
                        "invalid {}: expected not empty string",
                        stringify!($class),
                    )))
                } else {
                    Ok(Self(s))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn to_string(&self) -> String {
                self.0.clone()
            }
        }
    };
}
