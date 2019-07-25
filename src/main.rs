use tiny_http::{Server, Request, Response, Method, StatusCode};
use std::collections::HashMap;
use futures::future;

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
}

fn main() {
    // TODO: Hard code: port
    let server = Server::http("0.0.0.0:8080").unwrap();
    let mut path_to_sender  : HashMap<String, Request> = HashMap::new();
    let mut path_to_receiver: HashMap<String, Request> = HashMap::new();

    tokio::run(future::lazy(move || {
        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                     request.method(),
                     request.url(),
                     request.headers()
            );

            // Get path
            // TODO: Remove query parameters
            let path = request.url();

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
