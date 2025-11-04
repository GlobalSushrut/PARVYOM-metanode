//! Configuration module for BPCI Enterprise
//! 
//! Provides advanced environment configuration with:
//! - env.ini parser (human-readable INI format)
//! - env.toml parser (structured TOML format)
//! - envtoml.lock (lock file for reproducible deployments)
//! - vPod virtual environment support
//! - BSO-K8 orchestrator integration

pub mod env_ini_parser;

pub use env_ini_parser::{
    EnvIniParser,
    EnvIniConfig,
    EnvSection,
    EnvVariable,
    VarType,
    VPodEnvironment,
    BsoK8Config,
    EnvTomlLock,
};
