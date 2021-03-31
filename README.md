# rust-actix-web
This repository is a proposal for an OpenFaaS template that uses the actix-web framework in Rust.

## Actix Web

Actix web is a high-performance, feature rich web framework that uses the Rust programming language.

The small compiled binary size, coupled with extremely fast start up times make Actix an ideal choice when building HTTP functions that are powerful, yet lightweight.
- Website: https://actix.rs/

## Initialization
Start by creating a project directory, and pulling the template from this repository:
```sh
$ mkdir my-function
$ cd my-function
$ faas-cli template pull https://github.com/baremaximum/rust-actix-web-template#main
```

Then, create a new function by running:

```sh
$ faas-cli new test-function --lang rust-actix-web
```

This will create a 'test-funtion.yml' file in the current directory, as well as a new directory named 'test-function' that contains the function's code.

Edit the 'test-function.yml' file to ensure that the image tag points to a repository that is reachable from the Kubernetes cluster:

```yaml
version: 1.0
provider:
  name: openfaas
  gateway: http://127.0.0.1:8080
functions:
  test-function:
    lang: rust-actix-web
    handler: ./test-function
    image: YOUR_DOCKERHUB_USERNAME/test-function:latest # Change this line

```

## Handler Function Crate

When you create a function using faas-cli, it will create a new directory with the same name as your function. Here you will find a library crate that contains your handler function:

```rust
use actix_web::{post, HttpRequest, Responder};

#[post("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK" // Do whatever you want here
}
```

## Main Binary Crate

The handler function crate is run from a separate intallable binary crate. Unlike the function crate, the main binary crate does not get copied from the template into the function directory. If you want to do things that require making changes to the binary crate (e.g. adding middleware, application state, etc.), those changes can be made in the local version of the template. The local template can be found in the `template/rust-actix-web/main` directory that was created when the template was pulled. Alternatively, the template can also be forked in order to create a custom version.

## Build Args

This image accepts 3 custom build args that can be used to customize the behavior of the function at build time:

| Argument       | Behavior           | Default Value |
| ------------- |:-------------:| -----:|
| RUST_LOG     | Sets the log level | INFO |
| WORKER_POOL_SIZE     | Sets the number of workers listening for connections. Each worker runs in its own thread     |   1 |
| JSON_MAX_SIZE | Sets maximum JSON payload size in bytes for incoming requests.      |    4096 |


## Testing

The function crate can be tested by running `cargo test` from the function directory. The template includes a unit test for the handler function.

## Build and deploy

Assuming you have faas-cli, and it is logged in to a kubernetes cluster with openfaas, the function can be built and deployed by running: 

```sh
$ faas-cli up -f test-function.yml
```

## Example - Bean counter

This example demonstrates how to create a function that uses application state to keep track of a counter that can safely be read and modified by multiple threads. Users can send PATCH requests to the function with a 32 bit signed integer, and the function adds that value to the current count, and responds with the new count.

### Sample request:
**Method**: PATCH <br>
**Headers**: "Content-Type": "application/json"<br>
**Body**:
```json
  {
    "change": 18
  }
```

### Sample response body:
```json
  {
    "current_count": 18
  }
```

<em>./my-function/src/lib.rs</em>:
```rust
use actix_web::{patch, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::{AtomicI32, Ordering};

// Incoming request struct
#[derive(Debug, Serialize, Deserialize)]
pub struct BeanChange {
    pub change: i32,
}

// Change HTTP method to PATCH
#[patch("/")]
pub async fn handler(
    bean_count: web::Data<AtomicI32>, // application passes state to handler
    item: web::Json<BeanChange>, // deserialized json from request
) -> impl Responder {
    // add value from request to counter in memory
    let old_count = bean_count.fetch_add(item.change, Ordering::SeqCst);

    //build and send a response with the new counter
    let resp = json!({ "current_count": old_count + item.change });
    HttpResponse::Ok().json(resp)
}

```

<em>./my-function/cargo.toml</em>:
```toml
[dependencies]
actix-web = "3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

In order to set application state, some changes need to be made to the main binary crate in the template.

<em>./template/rust-actix-web/main/src/main.rs</em>:
```rust
use actix_web::{web, middleware, App, HttpServer};
use log::info;
use function;
use std::env;
use std::sync::atomic::AtomicI32;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // get worker pool size from env.
    let cnt = env::var("WORKER_POOL_SIZE");
    let mut worker_count: usize = 1;

    match cnt {
        Ok(cnt) => { 
            worker_count = cnt.parse::<usize>()
                .expect("Could not parse WORKER_POOL_SIZE. Value must parse to valid usize") 
        }
        Err(_) => info!("WORKER_POOL_SIZE not set. Using default value 1.")
    }

    // get max json size from env.
    let max = env::var("JSON_MAX_SIZE");
    let mut max_size: usize = 4096;

    match max {
        Ok(max) => { 
            max_size = max.parse::<usize>()
                .expect("Could not parse WORKER_POOL_SIZE. Value must parse to valid usize") 
        }
        Err(_) => info!("JSON_MAX_SIZE not set. Using default value 4096.")
    }

    // Create the counter variable
    let bean_counter = web::Data::new(AtomicI32::new(0));

    // Create and start the server
    HttpServer::new(move || 
        App::new()
        .app_data(bean_counter.clone()) // pass counter to handler
        .wrap(middleware::Logger::default())
        .data(web::JsonConfig::default().limit(max_size))
        .service(function::handler))
        .workers(worker_count)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
```