// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

//no semicolon at the end of expression, if not it will turn to a statement which then will not returns values
fn square(num: i32) -> i32 {
    num * num
}
