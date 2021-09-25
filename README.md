# rust notes
    
    rust入门看的书
  
    rust程序设计第二版 pdf版本
    精通rust第二版 邓世超翻译的，京东/当当都可以买到

# rust社区

	Rust语言中文社区：https://rust.cc/
	Rust 中文 Wiki：https://wiki.rust-china.org/

# rust学习资料

    Rust语言中文社区：https://rust.cc/
    Rust 中文 Wiki：https://wiki.rust-china.org/

    rust程序设计第二版
    https://kaisery.github.io/trpl-zh-cn/
    https://github.com/KaiserY/trpl-zh-cn
    https://github.com/wspl/trpl-zh-cn-pdf

# rust资料汇总

    https://github.com/shirdonliao/awesome-rust

# rust安装

	https://www.rust-lang.org/zh-CN/tools/install
	https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

	shell安装
	curl https://sh.rustup.rs -sSf | sh
    对于centos7安装请看 rust-centos7-install.md
    
    rust升级执行如下操作：
    rustup update
    
    rust版本查看
    rustc --version
    rustc 1.55.0 (c8dfcfe04 2021-09-06)
    升级到指定版本 rust update "1.55.0"
    
# 设置rust国内镜像

	国内提高访问速度，建议设置环境变量 
	export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
	export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

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

# rust mysql操作

	https://github.com/launchbadge/sqlx
	https://github.com/diesel-rs/diesel

# rust redis操作

	https://github.com/mitsuhiko/redis-rs

# rust web框架

	https://github.com/actix/actix-web
	web框架入门
	https://actix-web.budshome.com/_index.html

# rust library集合

	https://github.com/zsiciarz/24daysofrust/blob/master/SUMMARY.md
	http://zsiciarz.github.io/24daysofrust/
