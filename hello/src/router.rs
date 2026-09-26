use crate::parse_request_line;
use crate::response::Response;

pub fn route(request_line: &str) -> Response{
    let Some((method, path, _version)) = parse_request_line(request_line) else{
        return Response::BadRequest;
    };

    match (method.as_str(), path.as_str()) {
        ("GET", "/") => Response::Index,
        ("GET", "/about") => Response::About,
        ("GET", _) => Response::NotFound,
        _ => Response::MethodNotAllowed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect_request_line;
    use std::fs;

    fn route_fixture(name: &str) -> Response {
        let raw = fs::read_to_string(format!("tests/fixtures/{name}.txt"))
        .expect("couldn't read fixture file");
        let line = collect_request_line(&raw).expect("fixture has no lines");
        route(&line)
    }

    #[test]
    fn fixtures_route_as_expected() {
        let cases = [
            ("happy_path", Response::Index),
            ("many_headers", Response::Index),
            ("no_trailing_blank_line", Response::Index),
            ("unknown_method", Response::MethodNotAllowed),
            ("empty_request_line", Response::BadRequest),
            ("missing_http_version", Response::BadRequest),
            ("weird_whitespace", Response::BadRequest),
            ("path_traversal", Response::NotFound),
        ];

        for (name, expected) in cases {
            assert_eq!(route_fixture(name), expected, "fixture: {name}");
        }
    }

    #[test]
    fn about_page_routes() {
        assert_eq!(route("GET /about HTTP/1.1"), Response::About);
    }

    #[test]
    fn unknown_get_path_is_not_found() {
        assert_eq!(route("GET /nope HTTP/1.1"), Response::NotFound);
    }


}
