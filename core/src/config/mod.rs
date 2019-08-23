use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Platform {
  #[serde(rename = "test")]
  Test,
  iOS,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Target {
  platform: Platform,
  engine: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum RepoType {
  #[serde(rename = "local")]
  Local,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Repo {
  type: RepoType,
  source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Scope {
  repo: String,
  scopes: Option<Vec<Scope>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ThreadType {
  #[serde(rename = "webworker")]
  WebWorker,
  #[serde(rename = "main")]
  Main,
  #[serde(rename = "thread")]
  Thread,  
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ThreadDriver {
  #[serde(rename = "direct")]
  Direct,
  #[serde(rename = "default")]
  Default,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Thread {
  type: ThreadType,
  name: String,
  driver: ThreadDriver,
}

#[serde(untagged)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ScheduleTarget {
  Expression { repr: String },
  Multiple { list: Vec<ScheduleTarget> }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Schedule {
  repo: String,
  target: ScheduleTarget,
  min: u8,
  max: u8,
  #[serde(default="false")]
  debug: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Configuration {
  target: Target,
  repos: Vec<Repo>,
  scopes: Vec<Scope>,
  threads: Vec<Thread>,
  setup: Vec<Schedule>,
}

/// Builds configuration from a yaml file.
pub fn build_config_from_file (String file) -> Result<Configuration, serde_yaml::Error> {
  let content = fs::read_to_string(file)?;
  let deserialized = serde_yaml::from_str(&content)?;
  Ok(deserialized)
}