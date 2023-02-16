pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Article {
    fn default_init() -> Self {
        Article {
            headline: "headline".to_string(),
            location: "location".to_string(),
            author: "author".to_string(),
            content: "content".to_string(),
        }
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn test() {
    let article = Article::default_init();
    println!("{}", article.summarize());
}