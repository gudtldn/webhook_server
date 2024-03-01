mod commands;
mod event_info;
mod payload;
mod utils;

use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use commands::*;
use event_info::Event;
use log::{debug, info, warn};
use payload::Payload;
use utils::*;

#[post("/webhook")]
async fn index(req: HttpRequest, data: web::Json<Payload>) -> impl Responder {
    debug!("Received request: {:?}", req);
    let events = get_event_data();

    match Event::from_str(
        req.headers()
            .get("x-github-event")
            .unwrap()
            .to_str()
            .unwrap(),
    ) {
        Event::Push => {
            if let Some(action) = find_repositeory_event(&events, &data.repository) {
                info!("Received push event");
                if let Some(refer) = &data.r#ref {
                    let branch_name = refer.trim_start_matches("refs/heads/");
                    let allowed_branches = ["main", "master"];
                    if !allowed_branches.contains(&branch_name) {
                        return HttpResponse::NoContent();
                    }
                } else {
                    return HttpResponse::NoContent();
                }

                change_directory(action.path.as_str()).expect("Failed to change directory");
                pull_from_github().expect("Failed to pull from GitHub");
                if let Some(container_name) = &action.docker_container_name {
                    restart_container(container_name.as_str())
                        .expect("Failed to restart container");
                }
            }
        }
        Event::Ping => {
            info!("Received ping event");
        }
        _ => {
            warn!("NotImplemented");
        }
    }

    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging
    init_logger();

    println!("Starting webhook server");

    // Set up the webhook
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:7899")?
        .run()
        .await
}
