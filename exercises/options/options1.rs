// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    match maybe_number {
        Some(num) => println!("printing: {}", num),
        None => println!("None"),
    }
    if maybe_number.is_none() {
        println!("`maybe_number` is NOT a number")
    }
    if maybe_number.is_some() {
        println!("`maybe_number` is a number")
    }
    
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [Some(1); 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}
