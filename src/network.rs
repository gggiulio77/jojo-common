use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Ssid(String);

impl TryFrom<String> for Ssid {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 {
            Ok(Ssid(value))
        } else {
            Err(value)
        }
    }
}

impl Display for Ssid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Password(String);

impl TryFrom<String> for Password {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 64 {
            Ok(Password(value))
        } else {
            Err(value)
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct NetworkCredentials {
    pub ssid: Ssid,
    pub password: Password,
}

impl NetworkCredentials {
    pub fn new(ssid: Ssid, password: Password) -> Self {
        NetworkCredentials { ssid, password }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssid_validation() {
        let ok = Ssid::try_from("Network1234567891234567891234567".to_string());
        let error: Result<Ssid, String> =
            "Network12345678912345678912345678".to_string().try_into();
        let to_string = Ssid::try_from("Network".to_string()).unwrap().to_string();

        assert_eq!(true, ok.is_ok());
        assert_eq!(true, error.is_err());
        assert_eq!("Network".to_string(), to_string);
    }

    #[test]
    fn password_validation() {
        let ok: Result<Password, String> =
            "PasswordPasswordPasswordPasswordPasswordPasswordPasswordPassword"
                .to_string()
                .try_into();
        let error = Password::try_from(
            "PasswordPasswordPasswordPasswordPasswordPasswordPasswordPassword1".to_string(),
        );
        let to_string = Ssid::try_from("Password".to_string()).unwrap().to_string();

        assert_eq!(true, ok.is_ok());
        assert_eq!(true, error.is_err());
        assert_eq!("Password".to_string(), to_string);
    }
}
