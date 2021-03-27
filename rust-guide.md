# rust学习资料

	Rust语言中文社区：https://rust.cc/
	Rust 中文 Wiki：https://wiki.rust-china.org/

# rust安装

	https://www.rust-lang.org/zh-CN/tools/install
	https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

	shell安装
	curl https://sh.rustup.rs -sSf | sh

# rust国内镜像

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

# rust程序设计

	https://github.com/KaiserY/trpl-zh-cn
	https://github.com/wspl/trpl-zh-cn-pdf

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