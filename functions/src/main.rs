fn main() {
    println!("Hello, world!");
    another(3);
    another(-3);
    another_str("OOOO");
    let x = creepy_return(6);
    println!("6+1={x}");
}

fn another(x: i32){
    println!("Value is {x}");
}

fn another_str(x:&str){
    println!("Get that string! {x}")
}

fn creepy_return(x:i32) -> i32{
    x + 1
}