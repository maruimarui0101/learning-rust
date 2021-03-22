fn main() {
    let mut s = String::from("hello"); 

    s.push_str(", twitch"); // pushing a literal into a String type
    println!("Our string is: {}", s);

    // Integers are simple values and stored on the stack 
    // so in this case, y is a copy of 5.
    let x = 5;
    let y = x;

    // do stuff with s since it's valid here

    // An example of move
    let s1 = String::from("hello");
    let s2 = s1;

    // will not work as ownership of s1 has been moved to s2
    // println!("s1 is {}", s1) 
    println!("s1 is {}", s2)

    // An example of clone 
}

// this scope is now over and s no longer valid
