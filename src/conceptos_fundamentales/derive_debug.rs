// println!

#[derive(Debug)]
struct User {
	name: String,
	age: i32,
}

fn main() {

	let user1: User = User {
		name: "Caleb".to_string(),
		age: 22,
	};

	println!("hola {:?}", user1);

}
