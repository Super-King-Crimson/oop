pub mod post;
use post::Post; 

pub mod better_blog;

pub fn create() {
    //A post starts as an empty draft
    //When the draft is done, a reveiew is requested
    //When the post is approved, it gets published
    //Only published blogs allow printed text to be printed

    let mut post = Post::new();
    
    post.add_text("Ok i finally got my blog post up!!");

    println!("{}", post.get_content());

    post.request_review();

    println!("still {}ing nothing ", post.get_content());

    post.approve();

    //only NOW does this code work
    println!("{}", post.get_content());

    //As you can see, this code has some problems
        //it's repetitive and unintuitive: request_review and approve have the exact same body fn
        //it's boilerplatey: every state is an object, every transition needs to have custom implementations of most of the State trait, 
            //because we can't return self (Box<dyn Trait> size not known at compile time)
        //it doesn't take advantage of rustc: if you miss a state transition you won't know unless your unit tests catch 'em

    //Instead, we can use rust types structs and encoding for different types
    let draft = better_blog::Post::new("Hello, World");

    let submitted_draft = draft.submit_for_approval();

    //not good enough, reject!
    let mut draft = submitted_draft.reject();

    //ok fine i'll be a bit more creative
    draft.add_text("\n\
    -Sincerely, the moon");

    let submitted_draft = draft.submit_for_approval();

    //there we go i like this
    let better_post = submitted_draft.approve();

    better_post.read();

    //By using Rust's type system, it is now completely impossible for users to try to read a drafted blog, or submit a posted blog
}