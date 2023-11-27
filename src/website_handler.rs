use super::http::{Request, RequestError, RequestMethod, Response, StatusCode};
use std::fs;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_error_request(&mut self, e: &RequestError) -> Response;
}

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // Make more secure for path by using 'fs::canonicalize'
        match fs::canonicalize(path) {
            Ok(path) => {
                fs::read_to_string(path).ok()

                // if path.starts_with(&self.public_path) {
                // } else {
                //     println!("Directory Traversal Attach Attempted: {}", file_path);
                //     None
                // }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            RequestMethod::Post => todo!(),
            RequestMethod::Get => match req.path() {
                "/" => Response::ok(self.read_file("index.html")),
                "/hello" => Response::ok(self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::ok(Some(content)),
                    None => Response::not_found(),
                },
            },
            RequestMethod::Put => todo!(),
            RequestMethod::Delete => todo!(),
            RequestMethod::Head => todo!(),
            RequestMethod::Connect => todo!(),
            RequestMethod::Patch => todo!(),
            RequestMethod::Trace => todo!(),
            RequestMethod::Options => todo!(),
        }
    }

    fn handle_error_request(&mut self, e: &RequestError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
