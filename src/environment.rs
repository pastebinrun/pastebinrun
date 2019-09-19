use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref ENVIRONMENT: Enviroment = match env::var_os("ENVIRONMENT") {
        Some(env) if env == "production" => Enviroment::Production,
        None => Enviroment::Development,
        environment => panic!("Unexpected environment: {:?}", environment),
    };
}

#[derive(Eq, PartialEq)]
enum Enviroment {
    Development,
    Production,
}

pub fn is_production() -> bool {
    *ENVIRONMENT == Enviroment::Production
}
