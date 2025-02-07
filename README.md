# Clienter

Clienter is a project designed to manage client interactions and data efficiently. This README provides an overview of the project, how to set it up, and how to contribute.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [License](#license)

## Installation

To install the project, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/clienter.git
   ```
2. Navigate to the project directory:
   ```sh
   cd clienter
   ```
3. Build the project:
   ```sh
   cargo build
   ```

## Usage

To run the project, use:

```sh
cargo run
```

To run the tests:

```sh
cargo test
```

## Examples

Here are some examples of how to use the `clienter` library:

### Simple GET Request

```rust
use clienter::{HttpClient, HttpMethod};

fn main() {
    let client = HttpClient::new();
    let request = client.request(HttpMethod::GET, "http://httpbin.org/anything");
    let mut response = client.send(&request).unwrap();
    println!("Status: {}", response.status);
    let body = response.body_as_string().unwrap();
    println!("Body: {}", body);
}
```

### POST Request with JSON Body

```rust
use clienter::{HttpClient, HttpMethod, HttpRequest};

fn main() {
    let client = HttpClient::new();
    let mut request = client.request(HttpMethod::POST, "http://httpbin.org/post");
    request.set_body(r#"{"key": "value"}"#);
    let mut response = client.send(&request).unwrap();
    println!("Status: {}", response.status);
    let body = response.body_as_string().unwrap();
    println!("Body: {}", body);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
