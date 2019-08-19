use std::env;

use rust_web::http_headers;
extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
struct MainPage<'a> {
	// the mutability of a struct is in its binding (all or nothing)
    name: &'a str,
	qs: Option<&'a str>,
}

fn main() {
	//l = fread(buf, 1, CL, stdin);
    
    let mut main_page = MainPage { name: "*-name-#", qs: Some("asd"), };
	main_page.qs = None; 

	let _qs = "QUERY_STRING";
	//main_page.qs = env::var(qs).unwrap_or("ERROR".to_string()); 
	//main_page.qs = env::var(qs);

    http_headers();
    print!("{}", main_page.render().unwrap());
}
