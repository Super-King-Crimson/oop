use std::fmt::Debug;
use std::collections::HashMap;

pub fn explain() {
    println!("One downside to trait objects is how they interact with type inference");

    //Just like an empty vector causes a type inference error, 
        //because the compiler doesn't know what the T in Vec<T> is,
    
    //Let's say we make a vector of debug printable stuff:
    let mut _display_vec: Vec<Box<dyn Debug>> = vec![
        Box::new(190u32),
        Box::new("Hello, world!"),
        Box::new(true),
        Box::new(Box::new(HashMap::from([
            ('a', "eiou"), 
            ('か', "きくけこ"),
            ('ナ', "ニヌネノ"),
        ]))),
    ];

    //Now let's say we want to get a value based on the number in index 0
    //...WE know it's a number, but the compiler has no clue
    // let num = 200 - *display_vec[0];

    //In fact, it is IMPOSSIBLE to downcast a trait object into a more concrete type
    //except the Any trait
}