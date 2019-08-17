extern crate askama;
use askama::Template;
#[derive(Template)] // This will generate the code...

#[template(path = "hello.html")] // relative to templates dir in crate root

struct HelloTemplate<'a> { //name can be anythiung
	name: &'a str, //field name should match variable name
}

fn main() {
	let hello = HelloTemplate { name: "DSDSDSworldasd" };
	
	println!("Content-type: text/html\n\n");
	println!("{}", hello.render().unwrap());
}

