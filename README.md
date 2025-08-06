## RUST Generic HTTP Router

A flexible, generic HTTP router for Rust that loads routing definitions from a JSON file.

This library allows you to decouple your routing logic from your application code. Define your endpoints in a routes.json file, implement the corresponding handlers, and let the router handle the dispatch.

## Features

* Configuration-driven: Routes are loaded from a JSON file at startup.
* Dynamic Path Parameters: Supports routes like `/users/:id`.
* Handler Existence Checking: Returns a 501 Not Implemented if a route is defined in JSON but no handler is registered for it.
* Framework Agnostic: Uses the standard `http` crate types, making it easy to integrate with servers like `hyper`, `axum`, or `actix-web`.
* Strongly-Typed: Leverages Rust's type system for safe and robust handler implementation.

## How to Use

1. Define your routes in `routes.json`

Create a `examples/routes.json` file that defines your application's endpoints.

```json
{
    "endpoints": [
        {
            "method": "GET",
            "path": "/users",
            "controller": "users_controller::get_all",
            "description": "Get all users"
        },
        {
            "method": "GET",
            "path": "/users/:id",
            "controller": "users_controller::get_by_id",
            "description": "Get user by id"
        },
        {
            "method": "POST",
            "path": "/users",
            "controller": "users_controller::create",
            "description": "Create new user"
        }
    ]
}
```

2. Implement Your Handlers

Create handlers that implement the HttpHandler trait.

```rust
use generic_http_router::{HttpHandler, HttpRequest, HttpResponse};
use http::StatusCode;

// A handler for fetching a user by ID
struct GetUserByIdHandler;

impl HttpHandler for GetUserByIdHandler {
    fn handle(&self, req: HttpRequest) -> HttpResponse {
        if let Some(id) = req.params.get("id") {
            let body = format!("Fetching user with id: {}", id);
            HttpResponse::new(StatusCode::OK, body.into_bytes())
        } else {
            HttpResponse::new(StatusCode::BAD_REQUEST, b"User ID missing".to_vec())
        }
    }
}

// You would create similar structs for other handlers
struct GetAllUsersHandler;
impl HttpHandler for GetAllUsersHandler {
    fn handle(&self, _req: HttpRequest) -> HttpResponse {
        HttpResponse::new(StatusCode::OK, b"Returning all users".to_vec())
    }
}

struct CreateUserHandler;
impl HttpHandler for CreateUserHandler {
    fn handle(&self, req: HttpRequest) -> HttpResponse {
        // In a real app, you would parse the body
        let body_str = String::from_utf8_lossy(&req.body);
        println!("Creating user with body: {}", body_str);
        HttpResponse::new(StatusCode::CREATED, b"User created".to_vec())
    }
}
```

3. Initialize Router and Register Handlers

In your `main.rs`, create a `Router`, register your handlers, and integrate it into your web server.

```rust
use generic_http_router::Router;
use std::sync::Arc;
// Import your handler implementations
// ...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Create a new router from the config file.
    let mut router = Router::new("routes.json")?;

    // 2. Register your handler implementations.
    //    The string key MUST match the "controller" value in routes.json.
    router.register("users_controller::get_all", Box::new(GetAllUsersHandler));
    router.register("users_controller::get_by_id", Box::new(GetUserByIdHandler));
    router.register("users_controller::create", Box::new(CreateUserHandler));

    // Note: If you forget to register a handler defined in JSON,
    // the router will correctly return a 501 Not Implemented error for that route.

    let shared_router = Arc::new(router);

    // ... integrate with your web server (e.g., Hyper) ...

    Ok(())
}
```

See the full example in `examples/simple_server.rs`.

## To Run the Example

```bash
# Make sure you have `routes.json` in the "examples" directory
cargo run --example simple_server
```

Then you can test it with curl:

```bash
curl http://127.0.0.1:3000/users
# Expected: Returning all users

curl http://127.0.0.1:3000/users/123
# Expected: Fetching user with id: 123

curl -X POST -d '{"name": "test"}' http://127.0.0.1:3000/users
# Expected: User created
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## Credits

This project is developed and maintained by [Carlos J. Ramirez](https://github.com/tomkat-cr). For more information or to contribute to the project, visit [rust-generic-http-router](https://github.com/tomkat-cr/rust-generic-http-router) on GitHub.
