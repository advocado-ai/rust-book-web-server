pub enum Response{
    Ok_200,
    BadRequest_400,
    NotFound_404,
    MethodNotAllowed_405,
}

impl Response{
    pub fn status_line(&self) -> &'static str{
        match self{
            Response::Ok_200 => "HTTP/1.1 200 OK",
            Response::BadRequest_400 => "HTTP/1.1 400 BAD REQUEST",
            Response::NotFound_404 => "HTTP/1.1 404 NOT FOUND",
            Response::MethodNotAllowed_405 => "HTTP/1.1 405 METHOD NOT ALLOWED",
        }
    }
    pub fn filename(&self) -> &'static str{
        match self{
            Response::Ok_200=>"public/hello.html",
            Response::BadRequest_400 => "error_pages/400.html",
            Response::NotFound_404 => "error_pages/404.html",
            Response::MethodNotAllowed_405 => "error_pages/405.html",

        }
    }
}