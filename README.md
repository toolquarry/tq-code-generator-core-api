# TQ Code Generator Core API

The TQ Code Generator Core API is a Rust API built with the warp web framework and the tq-code-generator-core crate. It generates random codes of a specified length.

## Prerequisites

Before running the API, make sure you have the following installed:

- Rust (stable)
- Cargo

## Installation

To install the API, follow these steps:

1. Clone this repository: `git clone https://github.com/toolquarry/tq-code-generator-core-api.git`
2. Navigate to the project directory: `cd tq-code-generator-core-api`
3. Build the project: `cargo build --release`

## Usage

To use the API, run the following command from the project directory:

```
cargo run --release
```

This will start the server on `localhost:8000`.

## Endpoints

The API has one endpoint:

- `GET /generate/{length}` - generates a random code of the specified length and returns it as a JSON response.

## Examples

To generate a code of length 8, send a GET request to `http://localhost:8000/generate/8`.

```
curl http://localhost:8000/generate/8
```

Response

```
{
  "code": "sdfKjMnE"
}
```

## Logging

The API uses the [rustic_logger](https://crates.io/crates/rustic-logger) crate for logging. Logs are written to `app-log.log` in the project directory.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
