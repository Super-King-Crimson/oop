pub mod blog;

pub fn explain() {
    println!("The state pattern is OOP");
    
    //We define a set of states a value can have, represented by state objects,
    //and the value's behavior changed depending on its state

    //These state objects share functionality,
    // and are responsible for their own behavior and when it should change its state

    //The value that holds a state object doesn't know what states do or when transitions should happen
    
    println!("The state pattern helps bc of encapsulation");
    //When the business requirements of a state object changes, 
    //only the code inside a state object must be changed

    //Neither the code holding the state object nor the code that uses the value has to be changed
    //Let's make a blog post workflow the OOP way, and then the Rust way
    blog::create();
}