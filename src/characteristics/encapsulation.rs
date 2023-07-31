pub fn explain() {
    println!("Encapsulation means implementation details are hidden from a user");

    //i.e. privacy rules. I can have private methods and fields that you can't use or change
    //This allows a developer to change an object's internals and not need to change the public API

    //Rust has that too, with pub (in fact Rust is private by default)
    let pub_struct = public_mod::PublicStruct::new(90);
    println!("{}", pub_struct.public_int);

    //god this is so boring
    // println!("{}", pub_struct.private_vec);
    pub_struct.print_vec();
}

pub mod public_mod {
    pub struct PublicStruct {
        pub public_int: i32,
        private_vec: Vec<String>,
    }

    impl PublicStruct {
        pub fn new(int: i32) -> PublicStruct {
            PublicStruct { 
                public_int: PublicStruct::super_secret_constructor_helper(int), 
                private_vec: vec!(String::from("haha"), String::from("you"), String::from("can't"), String::from("see"), String::from("me")),
            }
        }

        pub fn print_vec(&self) {
            println!("{:?}- OH WAIT HOW'D YOU SEE ME", self.private_vec);
        }

        fn super_secret_constructor_helper(data: i32) -> i32 {
            //we crunch the data by dividing it by 2
            data / 2
        }
    }
}