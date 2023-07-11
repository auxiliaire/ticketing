pub struct UserValidation;

impl UserValidation {
    pub fn password_validation(password: &str) -> Result<(), serde_valid::validation::Error> {
        match Self::is_password_valid(password) {
            true => Ok(()),
            false => Err(serde_valid::validation::Error::Custom(
                "Password error.".to_owned(),
            )),
        }
    }

    pub fn is_username_valid(name: &str) -> bool {
        name.len() > 6
    }

    pub fn is_password_valid(password: &str) -> bool {
        let numbers = "0123456789";
        let special_chars = ".%#*-_";
        let lower = "abcdefghijklmnopqrstuvwxyz";
        password.len() > 8
            && password.len() < 20
            && password.contains(|c| numbers.contains(c))
            && password.contains(|c| special_chars.contains(c))
            && password.contains(|c| lower.contains(c))
            && password.contains(|c| lower.to_uppercase().contains(c))
    }
}
