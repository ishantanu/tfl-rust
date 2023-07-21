# TFL API Wrapper

A rust crate for using the [Transport for London (TFL) API](https://api.tfl.gov.uk).

*Note: Only the Line API is currently supported. Other APIs are work in progress*

## Installation

Using `cargo`, add this to your project's `Cargo.toml`:
```toml
[dependencies]
tfl-api-wrapper = "0.1.3"
```

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

List valid modes
```rust
async fn list_valid_modes() {
    use tfl_api_wrapper::{Client, RequestBuilder};
    use std::env;

    let client = Client::new(env::var("APP_KEY").unwrap().into());
    let valid_modes = client.list_modes().fetch().await.unwrap();
}
```

List severity types
```rust
async fn list_severity_types() {
    use tfl_api_wrapper::{Client, RequestBuilder};
    use std::env;

    let client = Client::new(env::var("APP_KEY").unwrap().into());
    let severity_types = client.list_severity_types().fetch().await.unwrap();
}
```

List routes by mode
```rust
async fn list_routes_by_mode() {
    use tfl_api_wrapper::{Client, RequestBuilder, linemodels, models};
    use std::env;

    let client = Client::new(env::var("APP_KEY").unwrap().into());
    let modes: Vec<models::Mode> = vec![models::Mode::Bus, models::Mode::Tube];
    let routes = client
        .list_lines_routes_by_modes()
        .mode(modes)
        .fetch()
        .await
        .unwrap();
}
```

Get arrival predictions by lines
```rust
async fn get_arrivals_by_lines() {
    use tfl_api_wrapper::{Client, RequestBuilder, linemodels};
    use std::env;

    let client = Client::new(env::var("APP_KEY").unwrap().into());
    let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo, linemodels::LineID::Jubilee];
    let arrivals = client
        .arrival_predictions_by_lines()
        .line(lines)
        .fetch()
        .await
        .unwrap();
}
```

Fetch disruptions by mode
```rust
async fn get_disruptions_by_lines() {
    use tfl_api_wrapper::{Client, RequestBuilder, linemodels, models};
    use std::env;

    let client = Client::new(env::var("APP_KEY").unwrap().into());
    let modes: Vec<models::Mode> = vec![models::Mode::Bus, models::Mode::Tube];
    let disruptions = client
        .disruptions_by_mode()
        .mode(modes)
        .fetch()
        .await
        .unwrap();
}
```

## Tests
You can run the tests by running:
```sh
APP_KEY=hjdhajsdas cargo test
```

## Implemented APIs
- Version - Shows the API version
- Line
    - Get all valid routes for all lines, including the name and id of the originating and terminating stops for each route.
    - Get all valid routes for given line ids, including the name and id of the originating and terminating stops for each route.
    - Get disruptions for all lines of the given modes.
    - Get disruptions for the given line ids.
    - Get the list of arrival predictions for given line ids based at the given stop
    - Gets a list of the stations that serve the given line id
    - Gets a list of valid disruption categories
    - Gets a list of valid modes
    - Gets a list of valid ServiceTypes to filter on
    - Gets a list of valid severity codes
    - Gets all lines and their valid routes for given modes, including the name and id of the originating and terminating stops for each route
    - Gets all valid routes for given line id, including the sequence of stops on each route.
    - Gets lines that match the specified line ids
    - Gets lines that serve the given modes.
    - Gets the line status for all lines with a given severity
    - Gets the line status for given line ids during the provided dates e.g Minor Delays
    - Gets the line status of for all lines for the given modes
    - Gets the line status of for given line ids e.g Minor Delays
    - Gets the timetable for a specified station on the give line
    - Gets the timetable for a specified station on the give line with specified destination
    - Search for lines or routes matching the query string

## Limitations
- Currently, the enum types created for Lines does not contain buses starting with numeric characters (e.g. 101).

## References/Credits
1. Existing tfl wrappers
   1. [tfl-api-wrapper](https://github.com/ZackaryH8/tfl-api-wrapper) - NodeJS wrapper for TFL API, made with TypeScript.
   2. [tfl-api-wrapper-py](https://github.com/ZackaryH8/tfl-api-wrapper-py) - Python wrapper for TFL API
   3. [go-tfl](https://github.com/ZackaryH8/go-tfl) - Go client for TFL API
2. [Adzuna-rs](https://github.com/kamui-fin/adzuna-rs) - For existing wrapper implementation for Adzuna API.