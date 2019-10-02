use iron::prelude::*;
use iron::{status, Request};
use std::collections::HashMap;
use std::io::Read;
use std::result::Result;
use url;
use urlencoded::UrlEncodedQuery;
pub mod security;

/// Extracts the parameters out of the body of a post request and returns them as a hash map.
/// This empties the body of the request, so if you call it twice on the same request, the second
/// call will be an empty hash map.
pub fn get_body_parameters(r: &mut Request) -> HashMap<String, String> {
    let mut query = String::new();
    r.body
        .read_to_string(&mut query)
        .expect("unable to read post body to string");
    let to_return = url::form_urlencoded::parse(query.as_bytes());
    to_return.into_owned().collect::<HashMap<String, String>>()
}

/// Validates that query params exist in a URL from a get request.
pub fn validate_query_params(
    r: &mut Request,
    params: Vec<&str>,
) -> Result<HashMap<String, String>, IronError> {
    let query = r
        .get_ref::<UrlEncodedQuery>()
        .expect("invalid query in GET");
    for param in params.iter() {
        if !query.contains_key(*param) {
            // TODO this could be a better error
            return Err(IronError::new(
                iron::error::HttpError::Header,
                format!("error: missing parameter in query string: {}", param),
            ));
        }
    }
    let mut to_return = HashMap::<String, String>::new();
    for param in params.iter() {
        to_return.insert(
            param.to_owned().to_string(),
            query.get(*param).unwrap()[0].clone(),
        );
    }
    Ok(to_return)
}

/// Construct a basic Iron response for HTML content.
pub fn html_response(markup: String) -> Response {
    let mut resp = Response::with((status::Ok, markup));
    resp.headers.set(iron::headers::ContentType::html());
    resp
}
