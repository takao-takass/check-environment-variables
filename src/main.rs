#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, /*handlebars,*/ context};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/templating/<arg_foo>")]
fn templating(arg_foo: &str) -> Template {
    Template::render("index", context! {
        foo: arg_foo,        
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, templating])
        .attach(Template::fairing())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::uri;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert!(response.headers().get_one("X-Content-Type-Options").is_some());
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn templating() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/templating/sss").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }
}