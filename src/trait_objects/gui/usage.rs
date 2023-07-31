use super::{Draw, Screen};

pub fn explain() {
    println!("Ok cool we made a gui, how do we make components?");

    //Simple: put Drawable objects on the Screen
    let screen = Screen {
        components: vec![
            Box::new(TextBox {
                text: String::from("Hello, World!"),
                height: 10,
                width: 5,
            }),
            
            Box::new(SelectBox {
                height: 0,
                width: 0,
                options: vec![
                    TextBox {
                        text: String::from("Yes"),
                        height: 1,
                        width: 1,
                    },

                    TextBox {
                        text: String::from("No"),
                        height: 1,
                        width: 6,
                    },

                    TextBox {
                        text: String::from("Maybe???"),
                        height: 1,
                        width: 11,
                    },
                ]
            })
        ]
    };

    screen.run();
}

pub struct TextBox {
    text: String,
    height: u16,
    width: u16,
}

impl Draw for TextBox {
    fn draw(&self) {
        let mut acc = String::new();
        for _ in 0..self.height / 2 {
            acc.push('\n');
        }

        for _ in 0..self.width / 2 {
            acc.push(' ');
        }

        acc.push_str(&self.text);

        println!("{acc}");
    }
}
pub struct SelectBox {
    height: u16,
    width: u16,
    options: Vec<TextBox>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        for option in self.options.iter() {
            option.draw();
        }

        println!("My height is {}, my width is {}", self.height, self.width);
    }
}