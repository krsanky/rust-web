use std::env;
use webpage::http_headers;
extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
struct MainPage<'a> {
    name: &'a str,
    qs: Option<&'a str>,
    cl: &'a str,
    rm: &'a str,
}

fn main() {
    //l = fread(buf, 1, CL, stdin);

    let mut test_page = MainPage {
        name: "*-name-#",
        qs: None,
        cl: "",
        rm: "",
    };

    let cl = "CONTENT_LENGTH";
    let cl = env::var(cl);
    test_page.cl = match cl {
        Ok(ref v) => v,
        Err(_) => "None",
    };

    let rm = "REQUEST_METHOD";
    let rm = env::var(rm);
    test_page.rm = match rm {
        Ok(ref v) => v,
        Err(_) => "None",
    };

    let qs = "QUERY_STRING";
    let qs = env::var(qs);
    test_page.qs = match qs {
        Ok(ref v) => Some(v),
        Err(_) => None,
    };

    http_headers();
    print!("{}", test_page.render().unwrap());
}
