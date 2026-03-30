struct Tweet {
    content: String,
    author: String
}

impl Tweet {
    fn new(author: &str, content: &str) -> Self {
        Self { 
            content: String::from(content), 
            author: String::from(author),
        }
    }

    fn display(&self) {
        println!("Author: {}, Tweet: {}", self.author, self.content);
    }
}

fn main() {
    let mut my_tweet: Option<Tweet> = None;

    let a = String::from("Elon");
    let c = String::from("To the moon");

    my_tweet = Some(Tweet::new(&a, &c));

    if let Some(t) = &my_tweet {
        t.display();
    }
}