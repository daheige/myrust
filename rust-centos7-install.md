# rust centos安装
	
	https://www.cnblogs.com/Kempff/archive/2004/01/13/12705455.html

	1. 设置好清华镜像
		vim ~/.bashrc
		export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
	    export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup

		:wq
		source ~/.bashrc

	2. 下载rustup-init
		wget  'https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup/archive/1.21.1/x86_64-unknown-linux-gnu/rustup-init' -O 'rustup-init'
		chmod a+x rustup-init
		./rustup-init -v -y --no-modify-path
	3.修改~/.bashrc的配置文件
		# rust
		export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
	    export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
		export PATH="$HOME/.cargo/bin:$PATH"

		:wq
		source ~/.bashrc

		查看版本
		$ rustc --version
		rustc 1.49.0 (e1884a8e3 2020-12-29)

# rust镜像配置

	https://www.cnblogs.com/YMaster/p/13232965.html

	~/.cargo/config 

	# 指定镜像
	replace-with = 'tuna'

	# 源码地址
	[source.crates-io]
	registry = "https://github.com/rust-lang/crates.io-index"

	# 清华大学
	[source.tuna]
	registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

	# 中国科学技术大学
	[source.ustc]
	registry = "https://mirrors.ustc.edu.cn/crates.io-index"

	# 上海交通大学
	[source.sjtu]
	registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

	# rustcc社区
	[source.rustcc]
	registry = "git://crates.rustcc.cn/crates.io-index"

	[source.aliyun]
	registry = "https://code.aliyun.com/rustcc/crates.io-index"
