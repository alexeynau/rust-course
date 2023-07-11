// trait taht summarize info from different origins (tweet, articles)
pub trait Summary {
    // main method with default message
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    // get info about author
    fn summarize_author(&self) -> String;
    // get short summary
    fn summarize_short(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

}

// this struct defines a article
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// implementation trait for NewsArticle
impl Summary for NewsArticle {
    // defining a summary method
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // get author
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

}

// this struct defines a Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// implementation trait for Tweet
impl Summary for Tweet {
    // defining a summary method
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // get author
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// this function is the example of function that get structure that impements Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// examples of trait bound
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// clearer trait bounds with where clause
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

// this function returns summarizable struct
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}