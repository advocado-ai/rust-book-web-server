#[derive(Debug, PartialEq)]
pub enum Response{
    Index,
    About,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

impl Response{
    pub fn status_line(&self) -> &'static str{
        match self{
            Response::Index | Response::About => "HTTP/1.1 200 OK",
            Response::BadRequest => "HTTP/1.1 400 BAD REQUEST",
            Response::NotFound => "HTTP/1.1 404 NOT FOUND",
            Response::MethodNotAllowed => "HTTP/1.1 405 METHOD NOT ALLOWED",
        }
    }

    pub fn filename(&self) -> &'static str{
        match self{
            Response::Index => "public/hello.html",
            Response::About => "public/about.html",
            Response::BadRequest => "error_pages/400.html",
            Response::NotFound => "error_pages/404.html",
            Response::MethodNotAllowed => "error_pages/405.html",
        }
    }
    
}
