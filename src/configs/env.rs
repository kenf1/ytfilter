use std::env;

pub fn load_env_var(key: &str) -> Result<String, std::env::VarError> {
    env::var(key)
}
