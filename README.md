# rustlearn
learn rust demo
学习资源：
https://github.com/tyrchen/geektime-rust/tree/master
https://mp.weixin.qq.com/s/I1-7U0ACosnxvtaplKYXIQ

从零开始 用 Rust 语言写一个基于 RISC-V 架构的类 Unix 内核
https://rcore-os.cn/rCore-Tutorial-Book-v3/

# 显示当前安装的工具链信息
rustup show
# 检查安装更新
rustup update
# 卸载
rustup self uninstall
# 设置当前默认工具链
rustup default stable-x86_64-pc-windows-gnu
# 查看帮助
rustup -h
# -------------------------->配置工具链
# 查看工具链
rustup toolchain list
# 安装工具链
rustup toolchain install stable-x86_64-pc-windows-gnu
# 卸载工具链
rustup toolchain uninstall stable-x86_64-pc-windows-gnu
# 设置自定义工具链
rustup toolchain link <toolchain-name> "<toolchain-path>"
# -------------------------->配置一个目录以及其子目录的默认工具链
# 查看已设置的默认工具链
rustup override list
# 设置该目录以及其子目录的默认工具链
rustup override set <toolchain> --path <path>
# 取消目录以及其子目录的默认工具链
rustup override unset --path <path>
# -------------------------->配置工具链的可用目标
# 查看目标列表
rustup target list
# 安装目标
rustup target add <target>
# 卸载目标
rustup target remove <target>
# 为特定工具链安装目标
rustup target add --toolchain <toolchain> <target>
# -------------------------->配置 rustup 安装的组件
# 查看可用组件
rustup component list
# 安装组件
rustup component add <component>
# 卸载组件
rustup component remove <component>


rustc 相关
# 查看rustc版本
rustc --version
cargo 相关
# 查看cargo版本
cargo --version
# 新建项目
cargo new <project_name>
# 构建项目
cargo build
# 运行项目
cargo run
# 检查项目
cargo check
# 安装Rust二进制文件
cargo install
# 卸载Rust二进制文件
cargo uninstall
# 查看帮助
cargo -h
0x3. 配置工具链安装位置
在系统环境变量中添加如下变量：

CARGO_HOME 指定 cargo 的安装目录

RUSTUP_HOME 指定 rustup 的安装目录

默认分别安装到用户目录下的.cargo 和.rustup 目录

0x4. 配置国内镜像
1). 配置 rustup 国内镜像
在系统环境变量中添加如下变量：

# 中国科学技术大学
RUSTUP_DIST_SERVER：https://mirrors.ustc.edu.cn/rust-static
RUSTUP_UPDATE_ROOT：https://mirrors.ustc.edu.cn/rust-static/rustup
2). 配置 cargo 国内镜像
在 cargo 安装目录下新建 config 文件（注意 config 没有任何后缀），文件内容如下：

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'tuna'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
# 设置代理
# [http]
# proxy = "127.0.0.1:8889"
# [https]
# proxy = "127.0.0.1:8889"
0x5. Windows 交叉编译 Linux 程序
目标服务器是 Linux(CentOS 7) 64bit, 所以我们添加的 target 应该是x86_64-unknown-linux-gnu(动态依赖) 或者x86_64-unknown-linux-musl（静态依赖）

解释：

动态依赖：目标服务器需要包含动态依赖的相关库（用户共享库）

静态依赖，目标服务器不需要包含相应的库，但是打包文件会更大些

1). 添加需要的 target

rustup target add  x86_64-unknown-linux-musl

2). 在 cargo 安装目录下新建 config 文件（注意 config 没有任何后缀），添加的文件内容如下：

[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
3). 构建

cargo build --target x86_64-unknown-linux-musl
#rust学习日记
https://mp.weixin.qq.com/mp/appmsgalbum?action=getalbum&__biz=MzkwMjI5MjA5MA==&scene=1&album_id=2075607640214126593&count=3#wechat_redirect
#rust入门
https://mp.weixin.qq.com/mp/appmsgalbum?action=getalbum&__biz=MzU0MTg5MDMzOA==&scene=1&album_id=3187499705652690949&count=3#wechat_redirect
#rusut进阶
https://mp.weixin.qq.com/mp/appmsgalbum?__biz=Mzg5MDg4NTAzMg==&action=getalbum&album_id=3122274487372251145&scene=173&subscene=&sessionid=svr_d1b7bf56402&enterid=1703171241&from_msgid=2247484175&from_itemidx=1&count=3&nolastread=1#wechat_redirect
#rust中级
https://mp.weixin.qq.com/mp/appmsgalbum?action=getalbum&__biz=MzkwMjI5MjA5MA==&scene=1&album_id=2529793839025405952&count=3#wechat_redirect