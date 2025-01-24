use backend::main as application_main;
use cucumber::{given, then, when, World};
use reqwest::StatusCode;
use std::time::Duration;
use tokio::{task, time::sleep};

#[derive(Debug, Default, World)]
struct ApplicationWorld {
    status: Option<StatusCode>,
}

#[given("the main function has been called")]
async fn the_main_function_has_been_called(_: &mut ApplicationWorld) {
    task::spawn(async {
        application_main().await;
    });
}

#[when("I send a request")]
async fn i_send_a_request(world: &mut ApplicationWorld) {
    sleep(Duration::from_secs(2)).await;
    world.status = match reqwest::get("http://localhost:8000").await {
        Ok(res) => Option::from(res.status()),
        Err(_) => None,
    };
}

#[then("a response will be received")]
async fn a_response_will_be_received(world: &mut ApplicationWorld) {
    let code = world.status.unwrap();
    assert_eq!(StatusCode::NOT_FOUND, code);
}

#[tokio::main]
pub async fn main() {
    // Disabled because starting the application requires environment:
    // ApplicationWorld::run("tests/features").await
}
