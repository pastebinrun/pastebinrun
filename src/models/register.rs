use crate::schema::users::dsl::*;
use diesel::dsl::{exists, select};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use unicode_normalization::UnicodeNormalization;
use warp::Rejection;

#[derive(Deserialize, Default)]
pub struct Form {
    pub nickname: String,
    pub password: String,
    pub confirm_password: String,
}

impl Form {
    pub fn normalize(&mut self) {
        self.nickname = self.nickname.trim().into();
        self.password = self.password.trim().nfkc().collect();
        self.confirm_password = self.confirm_password.trim().nfkc().collect();
    }

    pub fn validate(
        &self,
        connection: &impl Connection<Backend = Pg>,
    ) -> Result<Vec<Issue>, Rejection> {
        let mut issues = Vec::new();
        if self.nickname.is_empty() {
            issues.push(Issue::MissingNickname);
        } else if select(exists(users.filter(nickname.eq(&self.nickname))))
            .get_result(connection)
            .map_err(warp::reject::custom)?
        {
            issues.push(Issue::NicknameAlreadyUsed)
        }
        if self.password.is_empty() {
            issues.push(Issue::MissingPassword);
        } else {
            if self.nickname == self.password {
                issues.push(Issue::PasswordTheSameAsNickname);
            } else if self.password.len() < 8 {
                // Yes, I'm checking byte length, that's intentional
                issues.push(Issue::PasswordShorterThanEightCharacters);
            }
            if self.confirm_password.is_empty() {
                issues.push(Issue::MissingConfirmPassword);
            } else if self.password != self.confirm_password {
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
    PasswordTheSameAsNickname,
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
            Self::PasswordTheSameAsNickname => "Your password is the same as your nickname.",
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

    fn random() -> String {
        let mut rng = thread_rng();
        (0..15).map(|_| rng.sample(Alphanumeric)).collect()
    }

    #[test]
    fn empty_everything_report() {
        assert_eq!(
            Form::default().validate(&POOL.get().unwrap()).unwrap(),
            &[Issue::MissingNickname, Issue::MissingPassword],
        );
    }

    #[test]
    fn different_passwords() {
        let random_username = random();
        let random_password = random();
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

    #[test]
    fn identical_nickname_and_password() {
        let random = random();
        assert_eq!(
            Form {
                nickname: random.clone(),
                password: random.clone(),
                confirm_password: random,
            }
            .validate(&POOL.get().unwrap())
            .unwrap(),
            &[Issue::PasswordTheSameAsNickname],
        );
    }

    #[test]
    fn normalization_applies_nfkc() {
        let random = random();
        let mut form = Form {
            nickname: String::new(),
            // LATIN SMALL LETTER E WITH ACUTE
            password: random.clone() + "\u{E9}",
            // LATIN SMALL LETTER E followed by COMBINING ACUTE ACCENT
            confirm_password: random + "e\u{301}",
        };
        form.normalize();
        assert_eq!(form.password, form.confirm_password);
    }
}
