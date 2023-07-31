pub mod usage;

pub fn build() {
    println!("Let's get this digital bread.");

    //Ok, step #1 is thinking about what our X is.
        //In an OOP language, we'd make a GuiPrototype class or something,
        //put some properties and methods in that,
        //and have all our other classes (buttons, etc) inherit from that

    //but, in our case... what shared traits (functions) will ALL gui elements have?

    //Well, they all have to be drawn and placed on a screen so...
    explain();
}

//This gui library has to have elements that all can be drawn, so let's make it a trait:
pub trait Draw {
    fn draw(&self) {}
}

//Then we'll make a screen object that holds a vector of objects that implement the Draw trait
pub struct Screen {
    //actually it has to be of pointers to Draw impls, wait for Chapter 19 for why we have to do that
    //We'll use the smart pointer Box for ownership, then the dyn keyword, to say 'any type with Draw impl'
    pub components: Vec<Box<dyn Draw>>,
}

//While structs and enums aren't objects, as they separate data and behavior,
    //trait objects are more like objects, as they COMBINE data and behavior
//However, data cannot be added to a trait object
//In that sense, trait objects just allow abstraction across common behavior

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn explain() {
    println!("Sometimes you HAVE to use trait objects");

    //Remember that chapter from generics where we could specify what type we wanted with impl Trait?
    //Look what happens if we try that here with generics:
    verify();
}

/*
    struct BadScreen {
        // impl Draw is only allowed in function and return types
        components: Vec<impl Draw>
    }
*/

//If we use T: Draw, then the code compiles, but...
struct MidScreen<T: Draw> {
    _components: Vec<T>
}

impl Draw for String {}
impl Draw for i32 {}

fn verify() {
    let _ = MidScreen {
        _components: vec![
            //We can't assign multiple types to the Vec, 
            //it'll only hold types that are the same as the first type
            String::from("Hello, World"),
            // 1i32,
            String::from("Wait does this even print"),
        ],
    };
}