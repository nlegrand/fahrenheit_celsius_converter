use std::io;

fn fahrenheit_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}

fn get_temperature() -> f64 {
    println!("Please type a number:");
    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");
    let t: f64 = t.trim().parse().expect("Please type a number");
    return t
}

fn main() {
    loop {
	println!("fahrenheit or celsius? [f/c]");
	let mut scale = String::new();

	io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");
	let scale: char = scale.trim().parse().expect("Please type f or c");
	if scale == 'f' {
	    println!("This is in fahrenheit scale");
	    let ft = get_temperature();
	    let ct = fahrenheit_to_celsius(ft);
	    println!("{ft} °F is {ct} °C");
	}
	else if scale == 'c' {
	    println!("This is in celsius scale");
	    let ct = get_temperature();
	    let ft = celsius_to_fahrenheit(ct);
	    println!("{ct} °C is {ft} °F");
	    
	}
    }
}
