//* Iterators */

fn main() {
	let s = [1, 2, 3];
	for x in s.iter() {
		println!("{}", x+1);
	}
	println!("{:?}", s);

	let mut c = Counter::new();
	// println!("{:?}", c);
	c.next();
	// println!("{:?}", c);

	let i = c.next();
	println!("{:?}", i);
	match i {
		Some(count) => println!("{:?}", count),
		None => println!("Llego al final"),
	}

}

#[derive(Debug)]
struct Counter {
	count: i32,
}

impl Counter {
	fn new() -> Counter {
		 Counter { count: 0 }
	}
}

// Trait = rasgos
impl Iterator for Counter {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {

		if self.count < 5 {
			self.count += 1;
			Some(self.count)
		} else {
			None
		}

	}
}