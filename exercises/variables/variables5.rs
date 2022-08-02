// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);

    //by declaring x again with "let", it creats a new variable 
    //the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
