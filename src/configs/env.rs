use std::env;

#[allow(dead_code)]
pub fn load_env_var(key: &str) -> Result<String, std::env::VarError> {
    env::var(key)
}
