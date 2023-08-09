pub struct DraftPost {
    text: String
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text)
    }

    pub fn submit_for_approval(self) -> AwaitingApprovalPost {
        AwaitingApprovalPost { text: self.text }
    }
}

pub struct AwaitingApprovalPost {
    text: String
}

impl AwaitingApprovalPost {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn approve(self) -> Post {
        Post { text: self.text }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { text: self.text }
    }
}

pub struct Post {
    text: String
}

impl Post {
    pub fn new(contents: &str) -> DraftPost {
        DraftPost { text: contents.to_string() }
    }

    pub fn read(&self) {
        println!("{}", self.text)
    }
}