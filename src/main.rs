use rust_web::http_headers;
extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
struct MainPage<'a> {
    name: &'a str,
}

fn main() {
    //let hello = HelloTemplate { name: "DSDSDSworldasd" };
    let main = MainPage { name: "*-name--" };

    http_headers();
    print!("{}", main.render().unwrap());
}
