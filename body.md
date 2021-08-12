## 我应该下载哪个？

架构-(忽略)-平台-ABI 以选择需要的二进制

Alpline等 linux 发行版用户请使用 `-musl` 。

iOS 的 [iSH](https://ish.app) 用户请使用 `i686-unknown-linux-musl`

### ❗️注意，程序运行时需要读取 `./res` 下的 `[0-4].png`
### 没有编译时读取，是为了方便用户（不会fork之后让CI build）自行修改手的图像
