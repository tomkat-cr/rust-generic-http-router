//! An example of how to use the `generic-http-router` library with `hyper`.

use generic_http_router::{HttpHandler, HttpRequest, HttpResponse, Router};
use http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

// --- Handler Implementations ---

/// A handler for fetching all users.
struct GetAllUsersHandler;
impl HttpHandler for GetAllUsersHandler {
    fn handle(&self, _req: HttpRequest) -> HttpResponse {
        HttpResponse::new(StatusCode::OK, b"Returning all users".to_vec())
    }
}

/// A handler for fetching a single user by their ID.
struct GetUserByIdHandler;
impl HttpHandler for GetUserByIdHandler {
    fn handle(&self, req: HttpRequest) -> HttpResponse {
        // Extract the 'id' parameter that `matchit` parsed for us.
        if let Some(id) = req.params.get("id") {
            let body = format!("Fetching user with id: {}", id);
            HttpResponse::new(StatusCode::OK, body.into_bytes())
        } else {
            // This case should ideally not be reached if the route is correct.
            HttpResponse::new(StatusCode::BAD_REQUEST, b"User ID missing".to_vec())
        }
    }
}

/// A handler for creating a new user.
struct CreateUserHandler;
impl HttpHandler for CreateUserHandler {
    fn handle(&self, req: HttpRequest) -> HttpResponse {
        // In a real application, you would deserialize the request body.
        println!(
            "Received request to create user with body: {}",
            String::from_utf8_lossy(&req.body)
        );
        HttpResponse::new(StatusCode::CREATED, b"User created".to_vec())
    }
}

/// The main service function that processes each incoming request.
async fn handle_request(
    hyper_req: Request<Body>,
    router: Arc<Router>,
) -> Result<Response<Body>, Infallible> {
    // Convert Hyper's request body into a Vec<u8> for our router.
    // Extract the body while preserving the request parts
    let (parts, body) = hyper_req.into_parts();
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();

    // Create the request for our router using the original parts
    let req_for_router = Request::from_parts(parts, body_bytes.to_vec());

    // Use the router to handle the request.
    let response = router.route(req_for_router);

    // Convert our router's response back into a Hyper response.
    let (parts, body) = response.into_parts();
    Ok(Response::from_parts(parts, Body::from(body)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Define the server address.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // --- Router Setup ---
    // 1. Create a new router from the config file.
    //    This assumes `routes.json` is in the root of the crate.
    let mut router = Router::new("./examples/routes.json")
        .expect("Failed to initialize router. Make sure './examples/routes.json' exists.");

    // 2. Register all your handler implementations.
    //    The string key MUST match the `controller` value in `routes.json`.
    router.register("users_controller::get_all", Box::new(GetAllUsersHandler));
    router.register("users_controller::get_by_id", Box::new(GetUserByIdHandler));
    router.register("users_controller::create", Box::new(CreateUserHandler));

    // Wrap the router in an Arc to share it safely across threads.
    let shared_router = Arc::new(router);

    // --- Hyper Server Setup ---
    // Create a service that uses our `handle_request` function.
    let make_svc = make_service_fn(move |_conn| {
        let router_clone = shared_router.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, router_clone.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server listening on http://{}", addr);
    println!("Try running:");
    println!("  curl http://{}/users", addr);
    println!("  curl http://{}/users/123", addr);
    println!(
        "  curl -X POST -d '{{\"name\":\"test\"}}' http://{}/users",
        addr
    );

    // Run the server.
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
