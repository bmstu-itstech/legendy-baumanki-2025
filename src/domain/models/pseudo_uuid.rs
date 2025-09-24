#[macro_export]
macro_rules! pseudo_uuid_impl {
    ($class:ident, $length:expr) => {
        impl $class {
            pub fn new() -> Self {
                Self(new_pseudo_uuid($length))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn to_string(&self) -> String {
                self.0.clone()
            }
        }

        impl TryFrom<String> for $class {
            type Error = DomainError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.len() != $length {
                    Err(DomainError::InvalidValue(format!(
                        "invalid {}: expected length = {}, got {}",
                        stringify!($class),
                        $length,
                        value.len()
                    )))
                } else {
                    Ok(Self(value))
                }
            }
        }
    };
}
