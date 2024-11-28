//* Flujos de control

fn main() {
	
	// if
	let num = 5;
	if num == 5 {
		println!("Es cinco");
	} else if num == 3 {
		println!("Es tres");
	} else {
		println!("No es cinco ni tres. :(");
	}

	let resultado = if num > 5 { 1000 } else { 0 };

	println!("resultado: {}", resultado);

	// Loop

	let mut counter = 5;

	let result = loop {
		if counter == 10 {
			break counter
		}
		counter += 1;
	};

	println!("result: {}", result);

	// while

	while counter > 0 {
		println!("result: {}", counter);
		counter -= 1;
	}

	// for

	let arreglo = [1, 2, 3, 4];

	for element in arreglo.iter() {
		println!("element: {}", element);
	}



}