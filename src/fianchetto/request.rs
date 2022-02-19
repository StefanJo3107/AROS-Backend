use serde_json::Value;

pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub content_type: &'a str,
    pub content_length: usize,
    pub content: serde_json::Value,
}

impl<'a> Request<'a> {
    pub fn new(request: &str) -> Result<Request, serde_json::Error> {
        let lines: Vec<&str> = request.split("\r\n").collect();
        let first: Vec<&str> = lines.get(0).unwrap().split(" ").collect();

        let mut method = "";
        if let Some(m) = first.get(0) {
            method = m;
        }
        let mut path = "";
        if let Some(p) = first.get(1) {
            path = p;
        }

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
            } else if line.contains("{") {
                content_started = true;
            }

            if content_started {
                content += line;
            }
        }
        let mut content_json = Value::Null;
        if content_type.to_lowercase().contains("json") {
            let json = content.trim_matches('\u{0}');
            content_json = serde_json::from_str(json)?;
        }

        Ok(Request {
            method,
            path,
            content_type,
            content_length,
            content: content_json,
        })
    }
}
