trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {} : and {}", self.username, self.content);
        content
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {} : and {}", self.author, self.content);
        content
    }
}

// Default value in trait
trait Code {
    fn get_nodes(&self) -> i32;
    fn total_nodes(&self) -> i32 {
        44 + self.get_nodes()
    }
}

struct MyCode {
    total_nodes: i32,
}

impl Code for MyCode {
    fn get_nodes(&self) -> i32 {
        self.total_nodes
    }
}

fn main() {
    let tweet_1: Tweet = Tweet {
        username: String::from("axshivam"),
        content: String::from("This is the sample content"),
        reply: false,
        retweet: false
    };

    let news_article_1 = NewsArticle {
        headline: String::from("Shiv Sharma"),
        location: String::from("Kanpur, Uttar Pradesh"),
        author: String::from("Shiv Sharma"),
        content: String::from("This is sample content"),
    };

    news_aggregator_tweet(&tweet_1);
    news_aggregator_news_article(&news_article_1);

    new_aggregator(&tweet_1);
    new_aggregator(&news_article_1);

    let total = MyCode {
        total_nodes: 300,
    };

    println!("Total Nodes: {}", total.total_nodes());

    get_news(&tweet_1);

    // mixup_news(&news_article_1, &tweet_1); // error 
    mixup_news(&news_article_1, &tweet_1); // with new function


}

fn new_aggregator(source: & impl Summary) {
    println!("{}", source.summarize());
}

fn get_news<T: Summary>(source: &T) {
    println!("Trait bound example: {}", source.summarize());
}


// fn mixup_news<T: Summary>(primary: &T, other: &T) {
//     println!("T1: {}", primary.summarize());
//     println!("T2: {}", other.summarize());
// }
fn mixup_news(primary: &impl Summary, other: &impl Summary) {
    println!("T1: {}", primary.summarize());
    println!("T2: {}", other.summarize());
}


// we can not use generics here
fn news_aggregator_tweet(tweet: &Tweet) {
    println!("There is a new news in the market");
    println!("The news is {} and it is published by {}", tweet.content, tweet.username);
}

fn news_aggregator_news_article(news: &NewsArticle) {
    println!("There is a new news in the market");
    println!("The news is {} and it is published by {}", news.content, news.author);
}
