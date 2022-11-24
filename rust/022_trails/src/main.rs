mod associated_types;
mod default;
mod impl_for_type;
mod r#dyn;

pub trait Summary {
    fn summarize(&self) -> String;
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
}

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

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    Tweet::notify(&tweet);
}

pub trait SharedTrait {
    fn need_to_implement(&self) -> &str;
    fn default_implementation(&self) -> &str {
        "Method is implemented in traits"
    }
}

pub struct Struct1 {}
impl SharedTrait for Struct1 {
    fn need_to_implement(&self) -> &str {
        "This method is implemented by struct 1"
    }
}

pub struct Struct2 {}
impl SharedTrait for Struct2 {
    fn need_to_implement(&self) -> &str {
        "This method is implemented by struct 2"
    }
}

#[cfg(test)]
mod tests {
    use crate::{SharedTrait, Struct1, Struct2};

    #[test]
    fn share_traits_methods() {
        let s1 = Struct1 {};
        // assert_eq!("Method is implemented in traits", s1.default_implementation());
        assert_eq!(
            "This method is implemented by struct 1",
            s1.need_to_implement()
        );

        let s2 = Struct2 {};
        // assert_eq!("Method is implemented in traits", s2.default_implementation());
        assert_eq!(
            "This method is implemented by struct 2",
            s2.need_to_implement()
        );
    }
}
