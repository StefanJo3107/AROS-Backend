pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub content_type: &'a str,
    pub content_length: usize,
    pub content: String,
}

impl<'a> Request<'a> {
    pub fn new(request: &str) -> Request {
        let lines: Vec<&str> = request.split("\r\n").collect();
        let first: Vec<&str> = lines.get(0).unwrap().split(" ").collect();

        let method = first.get(0).unwrap();
        let path = first.get(1).unwrap();

        let mut content_type = "";
        let mut content_length: usize = 0;
        let mut content_started = false;
        let mut content = String::from("");

        for line in lines {
            if line.contains("Content-Type") {
                let content_type_vec: Vec<&str> = line.split(":").collect();
                content_type = content_type_vec.get(1).unwrap();
                content_type = &content_type[1..];
            } else if line.contains("Content-Length") {
                let content_length_vec: Vec<&str> = line.split(":").collect();
                let mut length: &str = content_length_vec.get(1).unwrap();
                length = &length[1..];
                content_length = length.parse().unwrap();
                content_started = true;
                continue;
            }

            if content_started {
                content += line;
            }
        }

        Request {
            method,
            path,
            content_type,
            content_length,
            content,
        }
    }
}
