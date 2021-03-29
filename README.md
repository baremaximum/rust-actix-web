# rust-actix-web
This repository is a proposal for an OpenFaaS template that uses the actix-web framework in Rust.

## Actix Web

Actix web is a high-performance, feature rich web framework that uses the Rust programming language.

The small combiled binary size, coupled with extremely fast start up times make Actix an ideal solution when trying to build HTTP functions that are powerful, yet lightweight, and that can scale up or down very quickly.

- Website: https://actix.rs/

## Initialization
Start by creating a project directory, and pulling the template from this repository:
```sh
$ mkdir my-function
$ cd my-function
$ faas-cli template pull https://github.com/baremaximum/rust-actix-web-template
```

Then, create a new function based on this template:

```sh
$ faas-cli new test-function --lang rust-actix-web
```

This will create a 'test-funtion.yml' file in the current directory, as well as a new directory named 'test-function' that contains your function's code.

Edit the 'test-function.yml' file to ensure that the image name points to a repository that is reachable from your Kubernetes cluster:

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

At this point, the function is ready to be built and deployed.

## The Handler Function Crate

This template creates a library crate that publishes a handler function:

```rust
use actix_web::{post, HttpRequest, Responder};

#[post("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK" // Do whatever you want here
}
```

## The Main Binary Crate

The handler function crate is run from a separate intallable binary crate. Unlike the function crate, the main binary crate does not get copied from the template into the function directory. If you want to do things that require making changes to the binary crate (e.g. adding middleware), those changes can be made in the local version of the template, which can be found in the `template/rust-actix-web/main` directory that was created when the template was pulled.


## Testing

The function can be tested by running `cargo test` from the function directory. The template includes a basic unit test for the handler function.

## Build and deploy

Assuming you have faas-cli, and it is logged in to a kubernetes cluster with openfaas, the function can be built and deployed by running: 

```sh
$ faas-cli up -f test-function.yml
```
