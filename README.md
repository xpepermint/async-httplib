> Low-level HTTP helpers.

This crate is built on top of [async-std](https://github.com/async-rs/async-std) and provides common objects and helper functions for low-level HTTP operations.

**TO-DO:**

- Support Content-Type decoding (e.g. multipart/form-data) : https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type
- GZIP support for Transfer-Encoding: https://greenbytes.de/tech/webdav/rfc7230.html#header.transfer-encoding
- HTTP2: https://www.youtube.com/watch?v=r5oT_2ndjms, https://httpwg.org/specs/rfc7540.html (HPACK, PSAUDOs)
