pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    //not part of state pattern
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn get_content(&self) -> &str {
        // ""

        //We want what is returned by get_content to depend on the state, so we should make it a method on the state obj
        self.state
            .as_ref()
            .unwrap()
            .get_content(&self)
    }

    pub fn request_review(&mut self) {
        //Take is like clone for options but not really: it moves the Some value and leaves a None in the original place
        if let Some(s) = self.state.take() {
            //request_review takes ownership of the state, and since we own it now we can send it off to be consumed
            self.state = Some(s.request_review());
            //We use ownership and options to ensure Post can’t use the old state values
        }
    }

    //same structure here:
    pub fn approve(&mut self) {
        //move out state obj to convert state
        if let Some(s) = self.state.take() {
            //set old state to new, converted state
            self.state = Some(s.approve());
        }
    }
}

struct Draft {}

struct PendingReview {}

struct Published {}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    #[allow(unused)]
    fn get_content<'a>(&'a self, post: &'a Post) -> &'a str {
        ""
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//if your review is already pending and you request another one, nothing should happen
//state doesn't change, return self
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}
