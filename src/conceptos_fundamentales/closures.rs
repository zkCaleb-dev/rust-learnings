//* Closures */
fn main() {
	
	//* Closures: Una funcion que es definida en linea (inline) */
	//* | = pipe */
	let sum = |nro: i32| -> i32 {
		nro + 1
	};
	//* o tambien
	// let sum = |nro, nro2| {
	// 	nro + nro2
	// };
	//* o tambien
	let sum = |nro| {
		nro + 1
	};

	println!("{}", sum(10));


	let mut counter = 1;

	let mut incrementar = move || {
		counter += 1;
	};

	let variable = &counter;


	incrementar();

}
