pub mod http;
pub mod validations;
pub mod jwt;

pub fn setup_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
}