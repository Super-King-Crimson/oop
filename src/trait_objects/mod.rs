pub mod gui;
pub mod type_system_interference;
pub mod dynamic_dispatch;

pub fn explain() {
    println!("You definitely don't remember this, but remember Chapter 8?");

    //Let me grab some code from that lesson brb
    flashback();
    get_back_on_topic();

    //Ok let's make this Gui!
    gui::build();

    //then we'll talk about the downsides of trait objects
    type_system_interference::explain();

    //and whatever the heck this is!
    dynamic_dispatch::explain();
}

//Ok, remember when I said vectors can only store one type? Remember enums? Yeah.
#[derive(Debug)]
enum Number {
    Int(i64),
    Double(f64),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Text {
    String(String),
    Char(char),
}

#[derive(Debug)]
enum Types {
    Bool(bool),
    Text(Text),
    Number(Number),
}

fn flashback() {
    //We can now make a vector with multiple types!

    let vec = vec![
        Types::Bool(true),
        Types::Number(Number::Double(10.0)),
        Types::Text(Text::Char('願')),
        Types::Number(Number::Int(-1941)),
    ];

    println!("{vec:#?}");
}

fn get_back_on_topic() {
    println!("wow i really haven't improved much huh.");

    //IN ANY CASE, that code works. we have different types in an enum
    //but what if we wanted an extendable list of types? 
        //Like a 'any type works as long as it does X,' where X is something a user can implement?
    
    //Let's make an example Gui tool to work with this idea
    gui::build();
    gui::usage::explain();

    //The idea that we only care what a value responds to, and not what its internal values,
        //is called 🦆 typing (if it walks like a 🦆, talks like a 🦆, quacks like a 🦆, its a 🦆)
    
    //If you have a draw, you're a gui element, and you go on the screen
    //If you can't be drawn, you're not gui element, and you can't go on the screen
    
    //It's that simple!
}