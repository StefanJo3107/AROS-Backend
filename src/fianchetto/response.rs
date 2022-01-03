pub struct Response {}

impl Response {
    pub fn ok(contents: String) -> String {
        let status_line = "HTTP/1.1 200 OK";

        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        )
    }
}
