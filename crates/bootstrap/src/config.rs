use anyhow::{Context, Result};
use secrecy::SecretString;
use std::{env, net::SocketAddr, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: SecretString,
    pub log_filter: String,
    pub log_dir: Option<String>,
    pub jwt_secret: SecretString,
    pub jwt_issuer: String,
    pub jwt_ttl_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let jwt_secret = required("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            anyhow::bail!("JWT_SECRET minimal 32 karakter");
        }

        Ok(Self {
            bind_addr: parse("BIND_ADDR", "0.0.0.0:8080")?,
            database_url: SecretString::from(required("DATABASE_URL")?),
            log_filter: env::var("LOG_FILTER")
                .unwrap_or_else(|_| "info,myapp=debug,tower_http=debug".into()),
            log_dir: optional("LOG_DIR"),
            jwt_secret: SecretString::from(jwt_secret),
            jwt_issuer: required("JWT_ISSUER")?,
            jwt_ttl_secs: parse_or("JWT_TTL_SECS", 900)?,
        })
    }
}

fn required(key: &str) -> Result<String> {
    env::var(key)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("environment variable `{key}` wajib diisi"))
}

fn optional(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.trim().is_empty())
}

fn parse<T>(key: &str, default: &str) -> Result<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let raw = env::var(key).unwrap_or_else(|_| default.to_owned());
    raw.parse::<T>()
        .with_context(|| format!("nilai `{key}` tidak valid: `{raw}`"))
}

fn parse_or<T: FromStr>(key: &str, default: T) -> anyhow::Result<T> {
    match env::var(key) {
        Ok(v) => v
            .parse::<T>()
            .map_err(|_| anyhow::anyhow!("{key} harus berupa angka yang valid")),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(e) => Err(e.into()),
    }
}
