fn main() {
    println!("Hello, world!");
    task("abc",say);
    task("heige",hello);
    let mut i = incr(0,1);
    i = incr(i,2);
    println!("i = {}",i); // 3
    println!("1 in [1,2,3] = {}",find(2,&[1,2,3]));
    println!("5 in [1,2,3] = {}",find(5,&[1,2,3]));
    let (x,y) = muilt_value(1);
    println!("x = {},y = {}",x,y);

}

fn say(name :&str){
    println!("name is: {}",name)
}

fn hello(name :&str){
    println!("hell func call,name is {}",name)
}

// 将函数作为参数传递给另外一个函数
fn task(name :&str,func: fn(&str)){
    func(name)
}

fn incr(n :i32,step :i32)-> i32{
    n +step
}

// s是一个切片，在切片中查找指定元素
fn find(n:i32,s :&[i32]) -> bool{
    for i in s{ // i是&i32类型，迭代器
        if *i == n{ // 这里是取值操作，*i表示解引用操作
            return true;
        }
    }

    false
}

// 返回元祖模式，支持多个返回值
fn muilt_value(i:i32)->(i32,i32){
    (i +1,i*2 +1)
}