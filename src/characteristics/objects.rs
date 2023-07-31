pub fn explain() {
    println!("Objects Contain Data and Behavior");

    //According to the Gang of Four's book on OOP released in 1994, Rust is an OOP language
        //They said OO programs are made of objects, which package data and procedures to operate on that data

    //Let's look at a struct to further prove my point:
    let announcement = GymClass {
        the_fitnessgram_pacer_test: Test::ProgressivelyGetsMoreDifficult(true),
        kickball: false,
        i_cant_think_of_anything_else: None,
    }.get_pacer_test_announcement();

    println!("{announcement}");
}

pub struct AAAAAAAAA;
pub struct DangFr;
pub enum Test {
    MultiStage(u32),
    AerobicCapacity,
    ProgressivelyGetsMoreDifficult(bool),
    SecsUntilStart(u8),
}
use Test::*;

//Alright, we have our data in this struct
#[allow(unused)]
pub struct GymClass {
    the_fitnessgram_pacer_test: Test,
    kickball: bool,
    i_cant_think_of_anything_else: Option<DangFr>,
}

//And here's our methods, boom OOP section over go home
impl GymClass {
    pub fn get_pacer_test_announcement(&self) -> String {
        let mut str = String::from("The FitnessGram Pacer test is a multi-stage");
        let mut str2 = String::from("aerobic capacity test, that progressively gets more difficult");
        let mut str3 = String::from("as you continue. The FitnessGram Pacer Test will begin soon.");

        match self.the_fitnessgram_pacer_test {
            MultiStage(stages) => str.push_str(&format!(" ({stages} stages) ")),
            ProgressivelyGetsMoreDifficult(gets_difficult) => {
                if gets_difficult {
                    str2.push_str(" (i'm being so fr you better be prepared) ");
                } else {
                    str2.push_str(" (no it doesn't lmao) ");
                }
            }
            SecsUntilStart(secs) => str3.push_str(&format!(" ({secs} seconds) ")),
            _ => (),
        };

        return str + &str2 + &str3;
    }
}