> Low-level HTTP helpers

This crate is built on top of [async-std](https://github.com/async-rs/async-std) and provides common objects and helper functions for for low-level HTTP operations.

**Example:**

Reading HTTP request:

```rs
let (&mut version, &mut status) = (Vec::new(), Vec::new());
read_statusline(&mut stream, &mut request).await?;
let version = Method::from(&version)?;
let status = parse_status(&status)?;

let mut headers = HashMap::new();
loop {
    let (mut name, mut value) = (vec![], vec![]);
    match read_header(&mut stream, (&mut name, &mut value), None).await.unwrap() {
        0 => headers.insert(name, value),
        _ => break,
    };
}

let mut body = Vec::new();
let limit = Some(1024);
read_body(&mut stream, &mut body, limit).await?;
```

Reading HTTP response:

```rs
let (&mut version, &mut status) = (Vec::new(), Vec::new());
read_statusline(&mut stream, &mut request).await?;
let version = parse_method(&version)?;
let status = parse_status(&status)?;

let mut headers = HashMap::new();
loop {
    let (mut name, mut value) = (vec![], vec![]);
    match read_header(&mut stream, (&mut name, &mut value), None).await.unwrap() {
        0 => headers.insert(name, value),
        _ => break,
    };
}

let mut body = Vec::new();
let limit = Some(1024);
read_body(&mut stream, &mut body).await?;

let mut traimers = HashMap::new();
let limit = Some(1024);
read_headers(&mut stream, &mut traimers, limit).await?;
```

**TODO:**

