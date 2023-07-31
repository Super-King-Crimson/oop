pub fn build() {
    println!("Let's get this digital bread.");

    //Ok, step #1 is thinking about what our X is.
        //In an OOP language, we'd make a GuiPrototype class or something,
        //put some properties and methods in that,
        //and have all our other classes (buttons, etc) inherit from that

    //but, in our case... what shared traits (functions) will ALL gui elements have?

    //Well, they all have to be drawn and placed on a screen so...
}

//TODO
pub trait Draw {
    fn draw(&self);
}

//TODO
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}