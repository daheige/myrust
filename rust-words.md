# 3 Rust关键字分类

    按照方法进一步学习。

# 3.1 类型声明(Type Declaration)

    用于定义基础类型/复合类型/接口类型/元类型/函数类型等
    
        fn：声明一个函数
        const：声明一个常量，如const THING: u32 = 0xABAD1DEA;
        enum：声明一个枚举类型
        true/false：类型bool的值
        struct：声明结构体类型
        trait：声明一个trait，类似抽象接口
        union：声明一个联合体
        type：为存在类型定义一个别名
        let：定义变量（Bind a value to a variable）

    除了上述关键字可以自定义一些具体的类型之外，Rust还支持如下基础类型：
    
        array：[T; N]
        bool：布尔类型
        char：a single character，占四个字节，unicode
        f32/f64：32bit浮点、64bit浮点
        fn：函数指针类型
        i8/i16/i32/i64/i128，不同位宽的整数类型
        u8/u16/u32/u64/u128，不同位宽的无符号整数类型
        isize：指针宽带类型，如let n = -1 isize;
        usize： pointer-sized unsigned integer type
        references：A reference represents a borrow of some owned value
        slice：
        str：string literals
        tuple：A finite heterogeneous sequence,(T, U, ..)
        unit：()类似void

# 3.2 对象模型(Object Model)
    
    用来实现OOP和构建类型关系（继承，多态，接口）的关键字。

        struct：结构类型，可以使用impl定义成员函数
        trait：定义一组接口spec，可以被其他具体类型实现。
        impl：define implementations on types(struct/trait)，类似对象中成员函数
        dyn：highlight that calls to methods动态分发，用于实现虚函数类似语义
        self：referencing the current module and marking the receiver of a method.
        Self：implementing type within atraitorimplblock, or the current type
        as：Cast between types, or rename an import
    其中struct/trait/impl构成了Rust支持OOP编程范式的核心关键字。

# 3.3 控制（Control Flow）

    if
    continue
    break
    else
    for
    loop
    while
    in: Iterate over a series of values with for
    match:模式匹配
    return: 提前返回

# 3.4 内存模型（Memory Model）

    rust内存管理机制采用了ownership and the borrow checker来管理每个对象的生命周期。
    一些与内存管理相关的关键词：

    move：闭包变量传递，将外部引用/mut变量按值传递到闭包内
    mut：变量变为可变，rust默认变量不可“变”
    ref：refannotates pattern bindings to make them borrow rather than move
    static：变量声明周期
    unsafe ：不能被类型系统验证的内存安全的代码或者接口

# 3.5 并发模型（Concurrency Model）

    async
    await

    除了关键字，还实现了std::thread相关的库实现并发。

# 3.6 异常模型（Exception Model）

    用来处理错误和异常，不使用异常关键字；使用Result类型和panic!宏

# 3.7 语言互操作

    extern
    使用std::ffi库

# 3.8 模块化支持

    mod：Organize code into modules，可以嵌套
    crate：A rust二进制或者库被称为一个crate,extern crate;pub(crate);crate::foo::bar路径
    extern：extern crate一起使用，引用另外一个库
    super：The parent of the current module
    use：Import or rename items from other crates or modules.

# 3.9 其他

    where：限定
