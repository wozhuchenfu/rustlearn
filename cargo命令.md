cargo new [packagename] 创建新cargo project
cargo init 在目录文件下执行，初始化cargo project
cargo check 检验代码
cargo run 运行代码
cargo build 命令阻塞住时删除$HOME/.cargo目录下的.package-cache文件

在.cargo目录下创建config文件配置国内crate.io源

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
#replace-with = 'tuna' # 如：tuna、ustc，或者 rustcc 指定一个即可

# 中国科学技术大学
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
MacBook-Pro:.cargo jingyuqi$ ls -a
.		.DS_Store	bin		env
..		.package-cache	config		registry
MacBook-Pro:.cargo jingyuqi$ rm -rf .package-cache
MacBook-Pro:.cargo jingyuqi$ cat config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
#replace-with = 'tuna' # 如：tuna、ustc，或者 rustcc 指定一个即可

# 中国科学技术大学
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"