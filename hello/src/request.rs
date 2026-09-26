pub enum Request{
    GetIndex,
    Unknown(String),

}

impl Request{
    pub fn request_path(&self) -> String{
        match self{
            Request::GetIndex => "GET / HTTP/1.1".to_string(),
            Request::Unknown(path)=> path.to_string(),
        }
    }
}