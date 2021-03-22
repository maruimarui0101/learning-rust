fn main() {
    let mut s = String::from("hello"); 

    s.push_str(", twitch"); // pushing a literal into a String type
    println!("Our string is: {}", s);

    // Integers are simple values and stored on the stack 
    // so in this case, y is a copy of 5.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // do stuff with s since it's valid here

    // An example of move
    let s1 = String::from("hello");
    let s2 = s1;

    // will not work as ownership of s1 has been moved to s2
    // this is not a "Shallow copy" like in Python
    // println!("s1 is {}", s1) 
    println!("s1 is {}", s2);

    // An example of clone ("Deep copy")
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // known primitive types that can be stored on the Stack 
    // no difference between deep and shallow copying 

    let tup:(u32, bool) = (116, true);
    let tup_clone = tup;

    let (tup_x, _) = tup;
    let (tup_clone_x, _) = tup_clone;

    println!("tup)_x = {}, tup_clone_x = {}", tup_x, tup_clone_x);

    // Ownership and Functions 

    // Return Values and Scope 

    // ########################
    // References and Borrowing
    // ########################

    // Borrowing - references as function parameters 

    let s1 = String::from("hello");

    // normally passing s1 into a function transfers ownership to function Scope
    // but passing in the address reference of the variable instead (&)
    // opposite is dereferencing (*)

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable References 

    let mut s = String::from("hello");

    change(&mut s); // mutable reference to s 

    // CAN ONLY HAVE ONE MUTABLE REFERENCE PER VAR/VAL
    // allows mutation in a controlled manner, prevents DATA RACES 
    // e.g.

    // can use multiple mutable references with curly braces -> prevents simulataneous references 

    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;

    // cannot combine mutable and immutable references easily
    // e.g.

    let r1 = &s; // 
    let r2 = &s; // multiple immntable references are ok! 
    println!("{} and {}", r1, r2);
    // r1 and r2 no longer used! 
    // scope of immutable references end here! 

    // now ok to make a mutable reference, scopes do not overlap
    let r3 = &mut s; 
    println!("{}", r3)
    
    // ###################
    // Dangling references 
    // ###################

    // dangling pointer - pointing to a location in memory given to something else
    // , by freeing some memory while preserving a pointer to that memory. 

    // ###################
    // Rules of references 
    // ###################

    // 1 - can only have one mutable reference or a number of immutable ones 
    // 2 - all references must be valid! 


}

// this scope is now over and s no longer valid

fn calculate_length(s: &String) -> usize { // s = reference to String variable 
    s.len()
} // s goes out of scope but function never owns what it refers to 
// cannot drop a reference to a variable that is passed into a function 
// returning a value not needed, valur was never owned 

// ** will not work because cannot modify something that is being borrowed **

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ** will not work ** 

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!