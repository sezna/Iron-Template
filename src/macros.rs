
//! This module contains convenience macros for various repetitive tasks.
/// Conveniently create router middleware for iron.
#[macro_export]
macro_rules! routes {
    (
        $(
            $method:ident $path:expr => $destination:expr
         ),+
    ) => {
            {
                let mut router = Router::new();
                $(
                    match stringify!($method).to_string().as_str()  {
                        "get" =>  { router.get($path,  |r: &mut Request| $destination(r), $path); },
                        "post" => { router.post($path, |r: &mut Request| $destination(r), $path); },
                        _ => {}
                    };
                )+
                router
            }
    }
}