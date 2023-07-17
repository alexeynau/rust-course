//example of implementing an object-oriented pattern called "state"

// defines a post struct
pub struct Post {
    // every post has a state
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // create new post
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    // add text to post
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // get content
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // request review for post
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    // approve post to publish it
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
// this trait realize state pattern for struct it implement
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // ensuaring the draft content is empty
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

// A blog post starts as an empty draft.
impl State for Draft {
    
    // When the draft is done, a review of the post is requested.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // When the post is approved, it gets published.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Only published blog posts return content to print, 
    // so unapproved posts cant accidentally be published.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
