use clienter::{HttpClient, HttpMethod, HttpRequest, StatusCode};

#[test]
fn test_simple_request() {
    let client = HttpClient::new();

    let request = client.request(HttpMethod::GET, "http://httpbin.org/anything");
    let mut response = client.send(&request).unwrap();
    let body = response.body_as_string().unwrap();
    println!("Body: {}", body);

    assert_eq!(response.status, StatusCode::Ok200);
}
