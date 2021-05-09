pub trait State {
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

pub struct Post {
    state: Box<dyn State>,
    content: String
}

impl Post {
    pub fn new(content: &str) -> Post {
        Post {
            state: Box::new(Draft {}),
            content: String::from(content)
        }
    }

    pub fn content(&self) -> &str {
        self.state.content(self)
    }

    pub fn pending_review(&mut self) {
        self.state = Box::new(PendingReview {});
    }

    pub fn approve(&mut self) {
        self.state = Box::new(Approved {});
    }
}

struct Draft {}

impl State for Draft {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Approved {}

impl State for Approved {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
