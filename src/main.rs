extern crate askama;

use askama::Template;

#[derive(Template)] // This will generate the code...

#[template(path = "hello.html")] // relative to teamplstes dir in cratre root

struct HelloTemplate<'a> { //name can be anythiung
	name: &'a str, //firld name shiould mastch vbariable name
}

fn main() {
	let hello = HelloTemplate { name: "worldasd" };
	
	println!("Content-type: text/html\n\n");
	println!("{}", hello.render().unwrap());
}

