pub struct Response;

impl Response {
    fn response_format(status_line: &str, contents: String) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        )
    }

    pub fn ok(contents: String) -> String {
        let status_line = "HTTP/1.1 200 OK";

        Response::response_format(status_line, contents)
    }

    pub fn created(contents: String) -> String {
        let status_line = "HTTP/1.1 201 Created";

        Response::response_format(status_line, contents)
    }

    pub fn bad_request() -> String {
        let status_line = "HTTP/1.1 400 Bad Request";

        format!("{}\r\n\r\n", status_line)
    }

    pub fn bad_request_body(contents: String) -> String {
        let status_line = "HTTP/1.1 400 Bad Request";

        Response::response_format(status_line, contents)
    }

    pub fn not_found(contents: String) -> String {
        let status_line = "HTTP/1.1 404 Not Found";

        Response::response_format(status_line, contents)
    }
}
