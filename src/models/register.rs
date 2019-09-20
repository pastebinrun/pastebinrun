use crate::schema::users;
use diesel::dsl::{exists, select};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use warp::Rejection;

#[derive(Deserialize, Default)]
pub struct Form {
    pub nickname: String,
    pub password: String,
    pub confirm_password: String,
}

impl Form {
    pub fn validate(
        &self,
        connection: &impl Connection<Backend = Pg>,
    ) -> Result<Vec<Issue>, Rejection> {
        let mut issues = Vec::new();
        let nickname = self.nickname.trim();
        let password = self.password.trim();
        let confirm_password = self.confirm_password.trim();
        if nickname.is_empty() {
            issues.push(Issue::MissingNickname);
        } else if select(exists(users::table.filter(users::nickname.eq(nickname))))
            .get_result(connection)
            .map_err(warp::reject::custom)?
        {
            issues.push(Issue::NicknameAlreadyUsed)
        }
        if password.is_empty() {
            issues.push(Issue::MissingPassword);
        } else {
            if password.len() < 8 {
                // Yes, I'm checking byte length, that's intentional
                issues.push(Issue::PasswordShorterThanEightCharacters);
            }
            if confirm_password.is_empty() {
                issues.push(Issue::MissingConfirmPassword);
            } else if password != confirm_password {
                issues.push(Issue::PasswordsNotTheSame);
            }
        }
        Ok(issues)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Issue {
    MissingNickname,
    NicknameAlreadyUsed,
    MissingPassword,
    MissingConfirmPassword,
    PasswordShorterThanEightCharacters,
    PasswordsNotTheSame,
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingNickname => "The nickname cannot be empty.",
            Self::NicknameAlreadyUsed => "The name is already taken.",
            Self::MissingPassword => "The password cannot be empty.",
            Self::MissingConfirmPassword => "Please retype the password.",
            Self::PasswordShorterThanEightCharacters => "Password must be 8 or more characters.",
            Self::PasswordsNotTheSame => "Passwords must be identical",
        };
        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod test {
    use super::{Form, Issue};
    use crate::test::POOL;
    use rand::distributions::Alphanumeric;
    use rand::prelude::*;

    #[test]
    fn empty_everything_report() {
        assert_eq!(
            Form::default().validate(&POOL.get().unwrap()).unwrap(),
            &[Issue::MissingNickname, Issue::MissingPassword],
        );
    }

    #[test]
    fn different_passwords() {
        let mut rng = thread_rng();
        let random_username = (0..22).map(|_| rng.sample(Alphanumeric)).collect();
        let random_password: String = (0..22).map(|_| rng.sample(Alphanumeric)).collect();
        assert_eq!(
            Form {
                nickname: random_username,
                password: random_password.clone(),
                confirm_password: random_password + "a",
            }
            .validate(&POOL.get().unwrap())
            .unwrap(),
            &[Issue::PasswordsNotTheSame],
        );
    }
}
