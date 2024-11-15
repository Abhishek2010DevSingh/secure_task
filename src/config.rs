use std::{env, sync::LazyLock};

pub struct Config {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

fn get_env(key: &str, fallback: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        tracing::error!(
            "Environment variable `{}` not found. Using default value: `{}`",
            key,
            fallback
        );
        fallback.into()
    })
}

fn get_env_u16(key: &str, fallback: u16) -> u16 {
    env::var(key)
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or_else(|| {
            tracing::error!(
                "Environment variable `{}` not found or invalid. Using default value: `{}`",
                key,
                fallback
            );
            fallback
        })
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::from_env);

impl Config {
    pub fn from_env() -> Config {
        Config {
            port: get_env_u16("PORT", 3000),
            host: get_env("HOST", "127.0.0.1"),
            database_url: get_env("DATABASE_URL", "./test.db"),
        }
    }
}
