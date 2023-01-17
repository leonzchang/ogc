use crate::mono::Opts;

use std::{fs, path::PathBuf};

use clap::{Parser, ValueHint};
use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Parser)]
#[clap(name = "OGC", about = "OGame Cheater")]
pub struct Cli {
    #[clap(flatten)]
    pub base: BaseCli,

    #[clap(flatten)]
    pub shared_params: SharedParams,

    /// Subcommands
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
pub struct BaseCli {
    /// Executing Environment
    #[clap(short, long, env = "OGC_ENV", default_value = "dev")]
    pub env: Environment,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, EnumString, Display)]
pub enum Environment {
    #[strum(serialize = "dev", serialize = "develop")]
    Develop,
    #[strum(serialize = "stag", serialize = "staging")]
    Staging,
    #[strum(serialize = "prod", serialize = "production")]
    Production,
}

impl Environment {
    pub fn prod(&self) -> bool {
        matches!(self, Environment::Production)
    }

    pub fn dev(&self) -> bool {
        matches!(self, Environment::Develop)
    }

    pub fn staging(&self) -> bool {
        matches!(self, Environment::Staging)
    }
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Monolithic API
    Mono(Opts),
}

#[derive(Debug, Parser)]
pub struct SharedParams {
    /// Database connection
    #[clap(
        default_value = "postgres://postgres:123456@localhost:5432/ogc?sslmode=disable",
        env = "OGC_POSTGRES"
    )]
    pub database_url: String,

    /// Database Pool Size
    #[clap(default_value = "5", env = "OGC_POSTGRES_POOL_SIZE")]
    pub database_pool_size: u32,

    /// WebDriver URL
    #[clap(env = "OGC_WEBDRIVER_URL")]
    pub webdriver_url: Option<String>,

    /// Toml config path
    #[clap(
        short,
        long,
        env = "OGC_CONFIG_FILE",
        value_parser,
        value_hint = ValueHint::FilePath,
    )]
    pub config_path: Option<PathBuf>,
}
