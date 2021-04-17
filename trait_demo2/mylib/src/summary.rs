// 定义Summarizable trait一组行为方法
// 一个类型的行为由其可供调用的方法构成。
// trait是一组行为的集合
pub trait Summarizable {
    // fn summary(&self) -> String;

    // 可以像下面一样定义默认方法的实现
    fn summary(&self) -> String {
        format!("read more...")
    }

    fn author(&self) -> String;
}

#[derive(Debug)]
pub struct Article {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Article {
    pub fn new(headline: String, author: String, content: String) -> Article {
        Article {
            headline,
            author,
            content,
        }
    }
}

// Article 实现Summarizable trait(实现接口)
// 为类型实现 trait
// for 后面是需要实现 trait 的类型的名称
impl Summarizable for Article {
    fn summary(&self) -> String {
        format!(
            "type: art,author: {},headline:{},content:{}",
            self.author, self.headline, self.content
        )
    }

    fn author(&self) -> String {
        format!("@author:{}", self.author)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现Summarizable
// 编写函数体来为特定类型实现 trait 方法所拥有的行为
impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!(
            "type: tweet,headline:{},location:{},author:{},content:{}",
            self.headline, self.location, self.author, self.content
        )
    }

    fn author(&self) -> String {
        format!("@author:{}", self.author)
    }
}
