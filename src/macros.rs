
/*
routes!( get "/" => homepage, get "/about" => about, get "/contact" => contact );

... expands into ...

let mut router = Router::new();
router.get("/", |r: &mut Request| homepage(r), "/");
router.get("/about", |r: &mut Request| about(r), "/about");
router.get("/contact", |r: &mut Request| contact(r), "/contact");

*/
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