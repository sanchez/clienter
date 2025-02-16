use clienter::{HttpClient, HttpMethod};

pub fn main() {
    let client = HttpClient::new();

    let request = client.request(HttpMethod::GET, "http://httpbin.org/anything");
    let mut response = client.send(&request).unwrap();
    println!("Status: {}", response.status);

    println!("Headers:");
    for (key, value) in response.headers.iter() {
        println!("{}: {}", key, value);
    }

    let body = response.body_as_string().unwrap();
    println!("Body: {}", body);

    println!("Hello, world!");
}
