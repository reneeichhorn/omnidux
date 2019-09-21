use serde::{Serialize, Deserialize};
use std::fs;

fn default_as_false() -> bool {
  false
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Platform {
  #[serde(rename = "test")]
  Test,
  #[serde(rename = "ios")]
  IOS,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
  pub platform: Platform,
  pub engine: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RepoType {
  #[serde(rename = "local")]
  Local,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Repo {
  #[serde(rename = "type")]
  pub repo_type: RepoType,
  pub source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Scope {
  pub repo: String,
  pub scopes: Option<Vec<Scope>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreadType {
  #[serde(rename = "webworker")]
  WebWorker,
  #[serde(rename = "main")]
  Main,
  #[serde(rename = "thread")]
  Thread,  
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreadDriver {
  #[serde(rename = "mpsc-fifo")]
  MPSC_FIFO,
  #[serde(rename = "direct")]
  Direct,
  #[serde(rename = "default")]
  Default,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread {
  #[serde(rename = "type")]
  pub thread_type: ThreadType,
  pub name: String,
  pub driver: ThreadDriver,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduleTarget {
  Expression (String),
  Multiple (Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduleScaleValue {
  Static (u8),
  Dynamic (String),
}

fn default_as_one () -> ScheduleScaleValue { ScheduleScaleValue::Static(0) }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
  pub repo: String,
  pub target: ScheduleTarget,
  #[serde(default="default_as_one")]
  pub min: ScheduleScaleValue,
  #[serde(default="default_as_one")]
  pub max: ScheduleScaleValue,
  #[serde(default="default_as_false")]
  pub debug: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
  pub name: String,
  pub target: Target,
  pub repos: Vec<Repo>,
  pub scopes: Vec<Scope>,
  pub threads: Vec<Thread>,
  pub setup: Vec<Schedule>,
}

#[derive(Debug)]
pub enum BuildError {
  IOError (std::io::Error),
  ParseError (serde_yaml::Error),
}

impl From<std::io::Error> for BuildError {
  fn from(error: std::io::Error) -> Self {
    BuildError::IOError(error)
  }
}

impl From<serde_yaml::Error> for BuildError {
  fn from(error: serde_yaml::Error) -> Self {
    BuildError::ParseError(error)
  }
}

/// Builds configuration from a yaml file.
pub fn build_config_from_file (file: String) -> Result<Configuration, BuildError> {
  let content = fs::read_to_string(file)?;
  let deserialized = serde_yaml::from_str(&content)?;
  Ok(deserialized)
}
/// Builds configuration from a yaml string.
pub fn build_config_from_str(content: &str) -> Result<Configuration, BuildError> {
  let deserialized = serde_yaml::from_str(content)?;
  Ok(deserialized)
}