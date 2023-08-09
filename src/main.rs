

use oop::*;

fn main() {
    topic::introduce();
    characteristics::explain();
    trait_objects::explain();
    oop_design_patterns::explain();
}

fn concat_all<'a>(
    iter: impl Iterator<Item = String> + 'a,
    s: &'a str
) -> impl Iterator<Item = String> + 'a {
    iter.map(move |s2| s2 + s)
}