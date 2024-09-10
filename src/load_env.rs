use std::env;

use dotenvy::dotenv;

use crate::errors::Res;

pub fn get_env_var(varname: &str) -> Res<String> {
    dotenv().ok();
    dotenvy::from_path(".env.local")
        .map_err(|err| format!("Failed to load `.env.local`: {err}"))?;
    env::var(varname).map_err(|err| format!("{varname} must be set: {err}"))
}
