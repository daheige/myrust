# rust 泛型的性能

    Rust 实现泛型的方式意味着你的代码使用泛型类型参数相比指定具体类型并没有任何速度上的损失！ 
    Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
    单态化是一个将 泛型代码转变为实际放入的具体类型的特定代码的过程

    我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。
    这意味 着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码 一样。
    这个单态化过程正是 Rust 泛型在运行时极其高效的原因。
