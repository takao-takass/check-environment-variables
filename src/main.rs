#[macro_use] extern crate rocket;
use rocket::response::status;
use rocket::http::{Status, ContentType};
use rocket_dyn_templates::{Template, /*handlebars,*/ context};

#[get("/")]
fn index() -> &'static str {
    get_environment_variables()
}

fn get_environment_variables() -> &'static str {
    let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    match env.as_str() {
        "development" => "Hello, world!",
        "production" => "Hello, world! (production)",
        _ => "Hello, world! (unknown environment)",
    }
}

#[get("/environment")]
fn environment() -> Template {
    let sashimi = std::env::var("CHECK_ENV_SASHIMI").unwrap_or_else(|_| "".to_string());
    let sushi = std::env::var("CHECK_ENV_SUSHI").unwrap_or_else(|_| "".to_string());

    Template::render("environment", context! {
        sashimi: sashimi,
        sushi: sushi,
    })
}

#[get("/json")]
fn json() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"hi\": \"world\" }"))
}

#[post("/<id>")]
fn accept(id: usize) -> status::Accepted<String> {
    status::Accepted(format!("id: '{}'", id))
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
        .mount("/", routes![index, templating, accept, json, environment])
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