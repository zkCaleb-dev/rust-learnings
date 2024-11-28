#[allow(unused_variables)] // Esto es para que no me alerte por variables que no estoy usando.
fn main() {
	// let-else
	let algun_numero: Option<i32> = Some(100);

	// * forma 1
	// match algun_numero {
	// 	Some(numero) => println!("numero valido: {}", numero),
	// 	None => println!("numero invalido"),
	// }

	// * forma 2
	// if let Some(numero) = algun_numero {
	// 	println!("numero valido: {}", numero);
	// } else {
	// 	println!("numero invalido");
	// }

	//* forma 3
	let Some(numero) = algun_numero else {
		panic!("El numero no es valido.");
	};

	println!("numero valido: {}", numero);

}