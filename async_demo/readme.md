# async
    
    异步函数

    函数开头加上 async 关键词就成为了异步函数。
    
    async fn function(argument: &str) -> usize {
        // ...
    }

    异步函数的行为和普通函数不同，当异步函数被调用时，内部的代码逻辑不会立即执行，
    相反，异步函数会返回一个匿名的 Future 类型，之后当我们 poll 这个 Future 的时候，
    函数内部的代码才会被执行并且执行到 await 处停止（如果异步函数内部有的话），直到异步函数结尾。
    
    异步函数其实是某种 delayed computation （延迟运算）—— 在手动 poll future 之前，
    异步函数内部一行代码也不会执行。

