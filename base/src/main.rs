// 静态量贯穿于整个程序的生命周期，因此任何存储在常量中的引用有一 个 'static 生命周期
static NAME: &'static str = "daheige";

fn main() {
    println!("Hello, world!");
    const U: i32 = 1;
    println!("U = {}", U);
    println!("NAME is {}", NAME);

    let m: i32 = 12;
    let n = m as i64; // 类型显式转化
    println!("m -> i64: {}", n);
    let x = 12;
    println!("x = {}", x);
    let (m, n) = (123, 12); // 变量的解构操作
    println!("m = {} n = {}", m, n);
    let mut i = 0;
    while i < 10 {
        println!("index = {}", i);
        i += 1;
    }

    for i in 1..5 {
        println!("current i = {}", i);
    }

    // for 迭代集合里面的元素
    let arr = [1, 12, 3, 4];
    for k in arr.iter() {
        println!("k = {}", k);
    }

    // 使用loop+if+break替代while
    let mut i = 0;
    loop {
        if i >= 100 {
            break;
        }
        println!("i = {}", i);
        i += 1;
    }

    // let ..if操作 这里的i被隐式覆盖
    let i = if i >= 10 { 5 } else { 10 };
    println!("i is {}", i);
    let mut str = String::from("daheige");
    str.push_str("hello");
    println!("str is {}", str);

    str = str + "ddd"; // 模拟字符串拼接
    println!("str is {}", str);

    let s = String::from("daheige"); // rust字符串底层式ptr,len,cap三部分组成，ptr指向存放字符串内存地址
    let s1 = s; // s移动到了s1,s离开了作用域后，自动释放内存
    println!("s1 is {}", s1);
    // println!("s is {}", s); // value borrowed here after move，离开了作用域后就无效，rust禁止使用无效的引用,这里会抛出异常

    // 实际使用clone的时候，特别消耗性能，堆上的数据被复制
    let s = String::from("hello,world");
    let s1 = s.clone();
    println!("s is {},s1 is {}", s, s1); // 可以打印，不会panic

    // 只在栈上的数据： 拷贝
    /*
     * 是像整型这样的在编译时已知大小的类型被整个储存在栈上，所以拷贝其实际的值是快速的。
     * 这 意味着没有理由在创建变量 y 后使 x 无效。换句话说，这里没有深浅拷贝的区别，
     * 所以这 里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它
     */
    let x = 1;
    let y = x;
    println!("x is {},y is {}", x, y);

    /*
    那么什么类型是 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则，
    任何简 单标量值的组合可以是 Copy 的，任何需要分配内存，或者本身就是某种形式资源的类型不会是 Copy 的。
    如下是一些 Copy 的类型：
    所有整数类型，比如 u32 。
    布尔类型， bool ，它的值是 true 和 false 。
    所有浮点数类型，比如 f64 。
    元组，当且仅当其包含的类型也都是 Copy 的时候。 (i32, i32) 是 Copy 的，不过 (i32, String) 就不是。
    */
    /* 所有权和函数
    将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样
    */

    let s = String::from("daheige");
    // 调用函数进入作用域scope
    say_hello(s);
    // 函数调用完毕后，s离开了作用域，value borrowed here after move
    // println!("s is {}", s);// 下面无法执行

    let s1 = String::from("heige");
    let s2 = get_val(s1); // s1传递给函数后，就离开了作用域，发生了移动
    println!("s2 is {}", s2);

    /*
    变量的所有权总是遵循相同的模式：将值赋值给另一个变量时移动它。
    当持有堆中数据值的变量离开 作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
    */

    let mut v = vec![1, 3, 4, 5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    // 通过let Some方式遍历
    let mut v = vec![1, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("{}", x)
    }
}

fn say_hello(s: String) {
    println!("s is {}", s);
}

fn get_val(s: String) -> String {
    s + "hello"
}
