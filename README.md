# rust notes
    
    rust入门看的书
    rust程序设计第二版 pdf版本
    精通rust第二版 邓世超翻译的，京东/当当都可以买到

# rust安装

https://www.rust-lang.org/zh-CN/tools/install

https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

	建议安装到rust v1.58.0+版本
    shell安装
	curl https://sh.rustup.rs -sSf | sh
    对于centos7安装请看 rust-centos7-install.md
    
    rust升级执行如下操作：
    rustup update
    
    rust版本查看
    cargo --version
    cargo 1.58.0 (7f08ace4f 2021-11-24)
    升级到指定版本 rust update "1.58.0"
    
# 设置rust国内镜像

	国内提高访问速度，建议设置环境变量 vim ~/.bashrc
	export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
	export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
    export PATH="$HOME/.cargo/bin:$PATH"
    
    :wq
    source ~/.bashrc

	在用户目录.cargo文件夹或在与Cargo.toml同级目录.cargo文件夹下创建config文件
	$cd ~/.cargo/
	$touch config
	添加如下内容：
	[source.crates-io]
	registry = "https://github.com/rust-lang/crates.io-index"
	# 指定镜像
	replace-with = 'ustc'

	# 清华大学
	[source.tuna]
	registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

	# 中国科学技术大学
	[source.ustc]
	registry = "git://mirrors.ustc.edu.cn/crates.io-index"

	# 上海交通大学
	[source.sjtu]
	registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

	# rustcc社区
	[source.rustcc]
	registry = "https://code.aliyun.com/rustcc/crates.io-index.git"

# 镜像设置参考
    https://mirrors.tuna.tsinghua.edu.cn/help/rustup/

# rust编辑器

    可以使用vscode,clion都可以
    对于vscode配置
    rust vscode setting.json配置

``` json
{
    "files.eol": "\n",
    "editor.formatOnSave": true,
    "editor.fontSize": 13,
    "workbench.colorTheme": "Monokai",
    "rust.all_features": true,
    "editor.formatOnPaste": true,
    "editor.multiCursorModifier": "ctrlCmd",
    "editor.snippetSuggestions": "top",
    "rust-client.channel": "stable",
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust"
    }
}
```

# vscode rust-analyer 插件
    选择rust-analyer作为Rust语言的插件，具体的安装很简单
    点击插件，选择安装即可，根据提示可能需要重新加载IDE
    在此，再推荐大家几个好用的插件：
        Better TOML，用于更好的展示.toml文件
        Error Lens, 更好的获得错误展示
        One Dark Pro, 非常好看的Vscode主题
        CodeLLDB, debugger程序

# rust社区
Rust语言中文社区 https://rustcc.cn/

# rust 编程
The Rust Programming Language 官方学习

https://doc.rust-lang.org/book/ch03-02-data-types.html

rust程序设计第二版
https://kaisery.github.io/trpl-zh-cn/

# rust语言圣经
https://course.rs/

https://github.com/sunface/rust-course

# Rust代码鉴赏
https://codes.rs/

https://github.com/sunface/rust-codes

# rust资料汇总
https://github.com/shirdonliao/awesome-rust

# rust 标注库设计分析
https://github.com/daheige/inside-rust-std-library

# rust mysql操作
https://github.com/launchbadge/sqlx

https://github.com/diesel-rs/diesel

# rust redis操作
https://github.com/mitsuhiko/redis-rs

# actix-web框架
https://github.com/actix/actix-web

web框架入门
https://actix-web.budshome.com/intro.html

https://actix.rs/docs/getting-started/

# tide web框架

https://github.com/http-rs/tide

https://blog.csdn.net/kk3909/article/details/107202731/

https://zhuanlan.zhihu.com/p/49293350

https://tide.budshome.com/02-server_routes_endpoints/00-intro.html

# future异步编程

https://www.bilibili.com/video/av375066968

# rust library集合

https://github.com/zsiciarz/24daysofrust/blob/master/SUMMARY.md

http://zsiciarz.github.io/24daysofrust/

# rust 编码规范
https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/
