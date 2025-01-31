use clienter::{HttpMethod, HttpRequest};

pub fn main() {
    let request = HttpRequest::new(HttpMethod::GET, "http://httpbin.org/anything");

    let response = request.execute().unwrap();

    println!("Hello, world!");
}
