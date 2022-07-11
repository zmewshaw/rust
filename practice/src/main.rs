fn main() {
    let x = String::from("Tenzin is so cute");
    println!("{}", x)
}

fn _do_something<'a>(x: &'a u64, _y: &'a u64) -> &'a u64 {
    println!("{}", x);
    x
}