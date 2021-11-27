use std::collections::HashMap;

// 编写一个宏,要先定义，然后才可以使用
// 元变量 $x 被解析成一个单独的表达式节点，并且在替换后依旧在语法 树中保持原值
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

/*
定义宏的使用语法，以及它展开后的形态。定 义方式类似match语句的语法，expander=>{transcriber}。
左边的是宏扩 展的语法定义，后面是宏扩展的转换机制。
语法定义的标识符以$开 头，类型支持item、block、stmt、pat、expr、ty、itent、path、tt
 */
// 自定义hashmap定义插入
macro_rules! hashmap {
    // *表示0个或多个
    // 支持重复多个这样的语法元素。我们 可以使用+模式和*模式来完成。
    // 类似正则表达式的概念，+代表一个或 者多个重复，*代表零个或者多个重复。
    // 因此，我们需要把需要重复的 部分用括号括起来，并加上逗号分隔符
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new(); // 初始化map
            $(map.insert($key, $val);)* // 插入多个值
            map
        }
    };
}

fn main() {
    println!("Hello, world!");
    println!("current file:{},line:{}", file!(), line!()); // 文件名，文件行数，编译期计算
    assert_eq!(25, five_times!(2 + 3));

    // let mut map = hashmap!["a"=>1,"b"=>2]; // macro调用方式[]或者()
    let mut map = hashmap!("a"=>1,"b"=>2);
    println!("map = {:?}", map);
    map.insert("c", 1);
    println!("map = {:?}", map);
    /*
    map = {"b": 2, "a": 1}
    map = {"b": 2, "c": 1, "a": 1}
     */
}
