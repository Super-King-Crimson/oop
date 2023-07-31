pub fn explain() {
    println!("You also don't remember this, but in Chapter 10 we talked about how generics have no overhead");

    //You'll also not recall the fact that Rust uses monomorphization to achieve this:
        //Basically generics gets turned into specific code on a case-by-case basis whenever they're used (check the book for more)

    println!("Unfortunately, this is NOT true for trait objects");

    //The code produced by generics and their monomorphization is doing static dispatch, 
        //where rustc knows what methods are being called at compile time
    //On the other hand, the code produced by trait objects does dynamic dispatch
        //Dynamic dispatch means that the compiler has NO CLUE what you're doing at compile time
        //Instead, the compiler emits some code that will allow the runtime to figure out what method to call
        //Dynamic dispatch also disallows rustc to inline a method’s code, preventing some optimizations
    
    //This means that trait objects actually do have a runtime cost in exchange for their flexibility

    //Since the compiler moves work to runtime in dynamic dispatch, using trait objects allows for:
        //Smaller binaries
        //Faster compiles
}