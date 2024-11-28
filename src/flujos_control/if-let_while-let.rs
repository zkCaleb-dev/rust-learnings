#[allow(unused_variables)] // Esto es para que no me alerte por variables que no estoy usando.
fn main() {
	// if-let

	let edad: Option<i32> = Some(20);

	if let Some(value) = edad {
		println!("edad: {}", value);
	}

	// while-let

	let mut mensajes_no_leidos = Some(100);
	
	loop {
		match mensajes_no_leidos {
			Some(value) => {
				if value > 0 {
					println!("tienes mensajes no leidos");
					mensajes_no_leidos = Some(value - 1)
				} else {
					println!("No hay mensajes nuevos");
					mensajes_no_leidos = None
				}
			},
			None => { break }
		}
	}


	while let Some(value) = mensajes_no_leidos {
		if value > 0 {
			println!("tienes mensajes no leidos");
			mensajes_no_leidos = Some(value - 1)
		} else {
			println!("No hay mensajes nuevos");
			mensajes_no_leidos = None
		}
	}

}