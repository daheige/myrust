# rust mod使用
    
    模块mod定义规则
    Rust 提供了将包分成多个 crate，将 crate 分成模块，
    以及通过指定绝对或相对路径从一个模块引用另一个模块 中定义的项的方式。
    你可以通过使用 use 语句将路径引入作用域，这样在多次使用时可以使用更短的路径。
    模块定义的代码默认是私有的，不过可以选择增加 pub 关键字使其定义变为公有

    默认所有的函数，类型等等都是私有的，除非使用pub关键字对外公开；
    如果一个叫做 foo 的模块没有子模块，应该将 foo 的声明放入叫做 foo.rs 的文件中。
    如果一个叫做 foo 的模块有子模块，应该将 foo 的声明放入叫做 foo/mod.rs 的文件 中。
    
    私有原则：
    1. 如果一个项是公有的，它能被任何父模块访问
    2. 如果一个项是私有的，它能被其直接父模块及其任何子模块访问
    
# 自定义模块使用

创建一个library
``` shell
  cargo new mylib --lib
```
    
配置文件中需要指定依赖关系
``` toml
[dependencies]
mylib = { path="./mylib"}
```

# 导入第三方lib
``` rust
extern crate rand; // 导入外部组件库
use rand::Rng;
// 在具体使用地方，例如下面使用方法
let secret_number = rand::thread_rng().gen_range(1, 10);
    
```
