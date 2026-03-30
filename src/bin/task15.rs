struct Tweet<'a> {
    content: &'a str,
    author: &'a str
}

impl<'a> Tweet<'a> {
    fn display(&self) {
        println!("Author: {}, Tweet: {}", self.author, self.content);
    }
}

fn main() {
    // let tweet = Tweet{
    //     content: "Hello world", 
    //     author: "Arsenii"
    // };

    // tweet.display();

    let mut my_tweet: Option<Tweet> = None;

    {
        let a = String::from("Elon");
        let c = String::from("To the moon");

        my_tweet = Some(
            Tweet {
                content: &c,
                author: &a,
            }
        );

        match my_tweet {
            None => println!("None"),
            Some(t) => t.display(),
        }
    }


}