fn main() {
    let text = "Hello world!";
    let s1 = String::from(text);
    // let s2 = s1;
    let s2 = s1.clone();
    
    println!("This prints from stack: {}", text);
    println!("This prints from heap (dynamic): {}", s1);
    // println!("This fails due to ownership: {}", s1);

    take_ownership(s1);

    // would fail because s1 was borrowed and then destroyed
    // println!("This prints from heap (dynamic): {}", s1);

    doesnt_take_ownership(&s2);
    println!("S2 is still valid: {}", s2);


}

fn take_ownership(s:String) {
    // s comes into scope here!
    println!("Printing from moved: {}", s);
}

fn doesnt_take_ownership(s:&String) {
    println!("Printing from reference: {}", s);
}