use std::io;

fn main() {
    let mut weight_str = String::new();

    io::stdin()
        .read_line(&mut weight_str)
        .expect("Ebalo si e maikata");

    let trimmed_str = weight_str.trim();

    // Try to parse as f32 first
    let parsed_float: Result<f32, _> = trimmed_str.parse();

    // If parsing as f32 fails, try parsing as i32 and then convert to f32
    let float_value = match parsed_float {
        Ok(value) => value,
        Err(_) => match trimmed_str.parse::<i32>() {
            Ok(int_value) => int_value as f32,
            Err(e) => {
                println!("Failed to parse string: {}", e);
                return;
            }
        },
    };

    match calculate_weight_on_mars(float_value) {
        x if x <= 20.0 => println!("Abe staash."),
        x if x > 20.0 && x < 60.0 => println!("Eto kolko kila si na Mars mrusna svinio: {}", x),
        _ => println!("Mamicata mu dea, izel si lunata she te eba."),
    }
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
