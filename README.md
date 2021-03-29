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
$ faas-cli template pull https://github.com/baremaximum/rust-actix-web-template
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

This template includes a library crate that publishes the handler function:

```rust
use actix_web::{post, HttpRequest, Responder};

#[post("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK" // Do whatever you want here
}
```

## Main Binary Crate

The handler function crate is run from a separate intallable binary crate. Unlike the function crate, the main binary crate does not get copied from the template into the function directory. If you want to do things that require making changes to the binary crate (e.g. adding middleware, application state, etc.), those changes can be made in the local version of the template. The local template can be found in the `template/rust-actix-web/main` directory that was created when the template was pulled. Alternatively, the template can also be forked in order to create a custom version.


## Testing

The function crate can be tested by running `cargo test` from the function directory. The template includes a unit test for the handler function.

## Build and deploy

Assuming you have faas-cli, and it is logged in to a kubernetes cluster with openfaas, the function can be built and deployed by running: 

```sh
$ faas-cli up -f test-function.yml
```
