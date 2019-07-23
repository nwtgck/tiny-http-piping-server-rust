use tiny_http::{Server, Request, Response, Method, StatusCode};
use std::collections::HashMap;


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

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        // Get path
        // TODO: Remove query parameters
        let path = request.url();

        // TODO: Use thread or something to handle multiple request
        match request.method() {
            &Method::Get => {
                match path_to_sender.remove(path) {
                    // If sender is found
                    Some(sender_req) => {
                        transfer(sender_req, request);
                    },
                    // If sender is not found
                    None => {
                        // Enroll the receiver request
                        path_to_receiver.insert(path.to_string(), request);
                    }
                };
            },
            &Method::Post | &Method::Put => {
                match path_to_receiver.remove(path) {
                    // If receiver is found
                    Some(receiver_req) => {
                        transfer(request, receiver_req);
                    },
                    // If sender is not found
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
    }
}
