pub mod aggregator;
use aggregator::*;
fn main() {
    // creating tweet instance
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    // test trait method for tweet
    println!("1 new tweet: {}", tweet.summarize());
    // creating article instance
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    // test trait method for article
    println!("New article available! {}", article.summarize());

    // test default impementation
    println!("1 new tweet: {}", tweet.summarize_short());
    println!("New article: {}", article.summarize_short());

}
