use mylib::common; // 引入自定义的包

fn main() {
    println!("Hello, world!");
    let a = 12;
    let b = 123;
    println!("max(12,123) = {}",common::max(a,b)); // 调用自定义的方法

    // 调用当前模块中的包
    let u = self::user::get_user("daheige".to_string(),29);
    println!("u = {:?}",u); // u = User { name: "daheige", age: 29 }
}

// 当前模块里的包user
mod user{
    #[derive(Debug)]
    pub struct User{
        name: String,
        age: i32,
    }

    // 公开的方法
    pub fn get_user(name :String,age:i32) -> User {
        User{
            name:name,
            age: age,
        }
    }
}
