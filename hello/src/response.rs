pub enum Response{
    Ok,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

impl Response{
    pub fn status_line(&self) -> &'static str{
        match self{
            Response::Ok => "HTTP/1.1 200 OK",
            Response::BadRequest => "HTTP/1.1 400 BAD REQUEST",
            Response::NotFound => "HTTP/1.1 404 NOT FOUND",
            Response::MethodNotAllowed => "HTTP/1.1 405 METHOD NOT ALLOWED",
        }
    }
    
}

pub enum ResponseRoute{
    Index,
    About,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

impl ResponseRoute{
    pub fn filename(&self) -> &'static str{
        match self{
            ResponseRoute::Index => "public/hello.html",
            ResponseRoute::About => "public/about.html",
            ResponseRoute::BadRequest => "error_pages/400.html",
            ResponseRoute::NotFound => "error_pages/404.html",
            ResponseRoute::MethodNotAllowed => "error_pages/405.html",
        }
    }
}
