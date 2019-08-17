extern crate askama;
use askama::Template;
#[derive(Template)] // This will generate the code...

// this decorates the struct
#[template(path = "hello.html")] 

struct HelloTemplate<'a> { //name can be anythiung
	name: &'a str, //field name should match variable name
}

fn main() {
	let hello = HelloTemplate { name: "DSDSDSworldasd" };
	
	println!("Content-type: text/html\n\n");
	println!("{}", hello.render().unwrap());
}

