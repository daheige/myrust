fn main() {
    let arr = [1,2,3];
    println!("arr = {}",arr[0]);
    let a = [3;5];
    println!("arr = {}",a[0]);
    println!("arr = {:?}",a); // [3, 3, 3, 3, 3]

    let num = 3;
    if num < 5{
        println!("num < 5");
    }else{
        println!("num >= 5");
    }

    let mut c = 0;
    // let ... loop用法
    let res = loop {
        if c >= 10{
            break c;
        }

        println!("current c = {}",c);
        c+=1;
    };

    println!("res = {}",res); // res = 10

    let mut n = 10;
    while n > 0 {
        println!("n = {}",n);
        n -=1;
    }

    // for in遍历
    for x in [1, 2, 3, 4].iter() {
    // for x in [1, 2, 3, 4] {
        println!("x = {}",x);
    }

    println!("========i rev========");
    for i in (1..5).rev() {
        println!(" = {}",i);
    }

    let mut s = String::from("hello,world");
    s.push_str(" heige ");
    s.push_str("rust");
    println!("s = {}",s);

    let s = String::from("hello");
    let s2 = s; // 将s移动到s2
    println!("s2 = {}",s2);
    // println!("s = {}",s); // ^ value borrowed here after move
    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    // 当持有堆中数据值的变量离开作用域时，其值将 通过 drop 被清理掉，除非数据被移动为另一个变量所有。


}
