pub mod objects;
pub mod inheritance;
pub mod encapsulation;

pub fn explain() {
    println!("Wait, what even is OOP");

    //Rust takes inspiration from several paradigms, like OOP and FP (remember closures and iterators?)
    //OOP's a bit broad, but usually they include objects, encapsulation, and inheritance
    //Does Rust have those?
    println!("I'm debating not doing a section for this I feel like it'll be boring");
    objects::explain();
    inheritance::explain();
    encapsulation::explain();
}