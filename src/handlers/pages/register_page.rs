use iron::prelude::*;
use templates;
use utils::html_response;
pub fn register(r: &mut Request) -> IronResult<Response> {
    Ok(html_response(
        templates::pages::generic::form::templates::register().render(r),
    ))
}
