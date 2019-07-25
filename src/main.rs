use tiny_http::{Server, Request, Response, Method, StatusCode};
use std::collections::HashMap;
use futures::future;

use structopt::StructOpt;

/// Piping Server in Rust
#[derive(StructOpt, Debug)]
#[structopt(name = "piping-server")]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Image width
    #[structopt(long, default_value = "8080")]
    http_port: u16,
}

fn transfer(mut sender_req: Request, receiver_req: Request) {
    let length = sender_req.body_length();
    let response = Response::new(
        StatusCode::from(200),
        vec![],
        sender_req.as_reader(),
        length,
        None
    );
    receiver_req.respond(response).unwrap();
    sender_req.respond(Response::from_string("[INFO] Sent successfully!\n")).unwrap();
}

fn main() {
    // Parse options
    let opt = Opt::from_args();
    // Get port
    let port = opt.http_port;

    let server = Server::http( ("0.0.0.0", port)).unwrap();
    let mut path_to_sender  : HashMap<String, Request> = HashMap::new();
    let mut path_to_receiver: HashMap<String, Request> = HashMap::new();

    tokio::run(future::lazy(move || {
        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                     request.method(),
                     request.url(),
                     request.headers()
            );

            // Create dummy root
            let dummy_root = url::Url::parse("file://").unwrap();
            // Parse request URL
            let uri = dummy_root.join(request.url()).unwrap();
            // Get path
            let path = uri.path();

            match request.method() {
                &Method::Get => {
                    // If a receiver has been registered already
                    if path_to_receiver.contains_key(path) {
                        let res =
                            Response::from_string(format!("[ERROR] Another receiver has been connected on '{}'.\n", path))
                                .with_status_code(400);
                        request.respond(res).unwrap();
                        continue;
                    }
                    match path_to_sender.remove(path) {
                        // If sender is found
                        Some(sender_req) => {
                            tokio::spawn(future::lazy(move || {
                                transfer(sender_req, request);
                                Ok(())
                            }));
                        },
                        // If sender is not found
                        None => {
                            // Enroll the receiver request
                            path_to_receiver.insert(path.to_string(), request);
                        }
                    };

                },
                &Method::Post | &Method::Put => {
                    // If a sender has been registered already
                    if path_to_sender.contains_key(path) {
                        let res =
                            Response::from_string(format!("[ERROR] Another sender has been connected on '{}'.\n", path))
                                .with_status_code(400);
                        request.respond(res).unwrap();
                        continue;
                    }
                    match path_to_receiver.remove(path) {
                        // If receiver is found
                        Some(receiver_req) => {
                            tokio::spawn(future::lazy(move || {
                                transfer(request, receiver_req);
                                Ok(())
                            }));
                        },
                        // If receiver is not found
                        None => {
                            // Enroll the sender request
                            path_to_sender.insert(path.to_string(), request);
                        }
                    };
                },
                _ => {
                    println!("Unsupported method: {}", request.method());
                }
            };
        };
        Ok(())
    }));
}
