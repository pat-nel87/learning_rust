// We declare a trait with the trait keyword
// We declared this trait as pub so that dependent crates can use of the trait
pub trait Summary {
    // here we declare the method signature
    // which describes the behavior of types that implement this trait
    fn summarize(&self) -> String;
    // Each type implementing this trait must provide its own custom behavior
    // for the body of the method
}
// Essentially the trait allows you to abstract a common behavior

// We can also define a default behavior for a trait's method
pub trait SummaryTwo {
    fn summarize(&self) -> String {
        String::from("Read stuff!..")
    }
}

// implementing a trait on a type..
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

