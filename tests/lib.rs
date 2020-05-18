use std::collections::HashMap;
use async_httplib::{read_first_line, read_header, read_exact, parse_method,
    parse_status, parse_version, Method, Status, Version};

#[async_std::test]
async fn reads_request() {
    let mut stream = String::new();
    stream.push_str("POST /path HTTP/1.1\r\n");
    stream.push_str("Host: google.com\r\n");
    stream.push_str("Content-Length: 5\r\n");
    stream.push_str("\r\n");
    stream.push_str("hello");
    let mut stream = stream.as_bytes();

    let (mut method, mut uri, mut version) = (vec![], vec![], vec![]);
    let size = read_first_line(&mut stream, (&mut method, &mut uri, &mut version), None).await.unwrap();
    assert_eq!(size, 21);

    let method = parse_method(method).unwrap();
    let uri = String::from_utf8(uri).unwrap();
    let version = parse_version(version).unwrap();
    assert_eq!(method, Method::Post);
    assert_eq!(uri, "/path");
    assert_eq!(version, Version::Http1_1);

    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let (mut name, mut value) = (vec![], vec![]);
        read_header(&mut stream, (&mut name, &mut value), None).await.unwrap();
        if name.len() == 0 {
            break;
        } else {
            headers.insert(
                String::from_utf8(name).unwrap(),
                String::from_utf8(value).unwrap(),
            );
        }
    }
    assert_eq!(headers.len(), 2);
    assert_eq!(headers.get("Host").unwrap(), "google.com");
    assert_eq!(headers.get("Content-Length").unwrap(), "5");

    let length = headers.get("Content-Length").unwrap().parse::<usize>().unwrap();
    let mut body = Vec::new();
    let size = read_exact(&mut stream, &mut body, length).await.unwrap();
    assert_eq!(size, 5);
    assert_eq!(String::from_utf8(body).unwrap(), "hello");
}

#[async_std::test]
async fn reads_response() {
    let mut stream = String::new();
    stream.push_str("HTTP/1.1 200 OK\r\n");
    stream.push_str("Content-Length: 5\r\n");
    stream.push_str("\r\n");
    stream.push_str("hello");
    let mut stream = stream.as_bytes();

    let (mut version, mut status, mut message) = (vec![], vec![], vec![]);
    let size = read_first_line(&mut stream, (&mut version, &mut status, &mut message), None).await.unwrap();
    assert_eq!(size, 17);

    let version = parse_version(version).unwrap();
    let status = parse_status(status).unwrap();
    assert_eq!(version, Version::Http1_1);
    assert_eq!(status, Status::Ok);

    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let (mut name, mut value) = (vec![], vec![]);
        read_header(&mut stream, (&mut name, &mut value), None).await.unwrap();
        if name.len() == 0 {
            break;
        } else {
            headers.insert(
                String::from_utf8(name).unwrap(),
                String::from_utf8(value).unwrap(),
            );
        }
    }
    assert_eq!(headers.len(), 1);
    assert_eq!(headers.get("Content-Length").unwrap(), "5");

    let length = headers.get("Content-Length").unwrap().parse::<usize>().unwrap();
    let mut body = Vec::new();
    let size = read_exact(&mut stream, &mut body, length).await.unwrap();
    assert_eq!(size, 5);
    assert_eq!(String::from_utf8(body).unwrap(), "hello");
}
