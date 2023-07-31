pub fn explain() {
    println!("No way rust has inheritance");
    
    //Yeah it doesn't lmaoooo
    
    //There ARE alternatives tho, depending on your use case:
        //If you want code reuse, just use traits
        //If you want to be able to use child class in places where a parent class is expected...
            //This is called polymorphism, where you can substitute a bunch of objects for each other 
            // at run-time as long as they have certain characteristics
        //You're a bad programmer LMAO
    
    //Inheritance apparently SUCKS, as it causes more shared code than necessary
//some subclasses don't need all from their parents, but they get it all: this makes less flexible programs
    //This can also cause dumb stuff, like being able to call methods on objects that don't make sense
    //Also some languages try to fix this by only allowing single inheritance, which...
        //further restricts a program's design flexibility
    
    //Rust chooses a different option: TRAIT OBJECTS!
    println!("Stick around for me trying to make a barebones ECS system w/trait objects");
}