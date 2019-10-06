use handlers;
use iron::prelude::*;
use templates;

pub fn log_in(r: &mut Request) -> IronResult<Response> {
    handlers::template(templates::pages::generic::form::templates::log_in().render(r))
}
