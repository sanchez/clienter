use clienter::{HttpMethod, HttpRequest, StatusCode};

#[test]
fn test_simple_request() {
    let request = HttpRequest::new(HttpMethod::GET, "http://httpbin.org/anything");

    let mut response = request.execute().unwrap();
    let body = response.body_as_string().unwrap();
    println!("Body: {}", body);

    assert_eq!(response.status, StatusCode::Ok200);
}
