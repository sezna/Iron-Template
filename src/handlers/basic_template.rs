use iron::prelude::*;
use iron::status;
/// A general template serving handler for any template that doesn't need any data.
pub fn template(t: String) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, t));
    resp.headers.set(iron::headers::ContentType::html());
    return Ok(resp);
}
