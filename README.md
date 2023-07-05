# TFL API Wrapper

A rust crate for using the [Transport for London (TFL) API](https://api.tfl.gov.uk).
[Cargo.toml](Cargo.toml)
*Note: This is currently WIP and not ready for use.*

## Installation

Using `cargo`, add this to your project's `Cargo.toml`:
```toml
[dependencies]
tfl-api-wrapper = "0.1.2"
```

## Implemented APIs
Currently only the version for the API is implemented.

## Usage

### Get the Keys from TFL API Portal
1. If you don't already have an account, register on [TFL API Portal](https://api-portal.tfl.gov.uk/).
2. Vist the [Products](https://api-portal.tfl.gov.uk/products) link and create a new subscription for `500 Requests per min` (allows 500 requests per min).
3. Next, visit your [profile](https://api-portal.tfl.gov.uk/profile), scroll down to the `Primary key` section of your subscription, click on `Show` and copy the value. You can use either of `Primary key` or `Secondary key`.

### Use the crate

Set the `APP_KEY` environment variable.

Instantiate the `Client` using:

```rust
use tfl_api_wrapper::{Client, RequestBuilder};
let client = Client::new(std::env::var("APP_KEY").unwrap());
```
Here `APP_KEY` could be either `Primary key` or `Secondary key`.

## Example

Get the API version:
```rust
async fn it_is_version_1() {
    use tfl_api_wrapper::{Client, RequestBuilder};
    let client = Client::new(std::env::var("APP_KEY").unwrap());
    let ver = client.api_version().fetch().await.unwrap();
}
```

## Tests
You can run the tests by running:
```sh
APP_KEY=hjdhajsdas cargo test
```

## References/Credits
1. Existing tfl wrappers
   1. [tfl-api-wrapper](https://github.com/ZackaryH8/tfl-api-wrapper) - NodeJS wrapper for TFL API, made with TypeScript.
   2. [tfl-api-wrapper-py](https://github.com/ZackaryH8/tfl-api-wrapper-py) - Python wrapper for TFL API
   3. [go-tfl](https://github.com/ZackaryH8/go-tfl) - Go client for TFL API
2. [Adzuna-rs](https://github.com/kamui-fin/adzuna-rs) - For existing wrapper implementation for Adzuna API.