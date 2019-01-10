# Rust学习笔记--第一天

## Cargo

### 创建一个工程

```.sh
cargo new 项目名
cd 项目名
```

第一行命令新建了名为 hello_cargo 的目录。我们将项目命名为 hello_cargo，同时 Cargo 在一个同名目录中创建项目文件。  
进入 hello_cargo 目录并列出文件。将会看到 Cargo 生成了两个文件和一个目录：一个 Cargo.toml 文件，一个 src 目录，以及位于 src 目录中 main.rs 文件

### Cargo.toml

```.rs
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["ping <843951249@qq.com>"]
edition = "2018"

[dependencies]

```

这个文件使用 `TOML` (Tom's Obvious, Minimal Language) 格式，这是 Cargo 配置文件的格式。

1. 第一行，[package]，是一个 **片段(section)** 标题，表明下面的语句用来配置一个包。随着我们在这个文件增加更多的信息，还将增加其他片段（section）.
2. 接下来的四行设置了 Cargo 编译程序所需的配置：项目的名称、版本和作者。Cargo 从环境中获取你的名字和 email 信息  
3. 最后一行，[dependencies]，是罗列项目依赖的片段的开始。在 Rust 中，代码包被称为 `crates`。这个项目并不需要其他的crate

`Cargo` 期望源文件存放在 src 目录中。项目根目录只存放 `README、license` 信息、配置文件和其他跟代码无关的文件。使用 Cargo 帮助你保持项目干净整洁，一切井井有条。

### 编译与运行

`cargo build`: 编译生成可执行文件,将运行的结果保存在`./target/debug`里面
`cargo run`: 编译并运行项目中的文件  
`cargo check`: 检查项目是否可以运行,并且不创建可执行文件,一般比`cargo build`快得多

## 一个简单的游戏

### let语句

`let foo=bar`:这行代码新建了一个叫做 `foo` 的变量并把它绑定到值 `bar` 上。在 Rust 中，变量默认是不可变的。下面的例子展示了如何在变量名前使用 mut 来使一个变量可变：  

```.rs
let foo = 5; // 不可变
let mut bar = 5; // 可变
```

### 使用Result类型来处理潜在的错误

```.rs
io::stdin.read_line(&mut guess).expect("Failed to read line");
```

`io::stdin.read_line(&mut guess)`的返回值是`Result`类型  
`Result`类型也就是一个枚举类型(enum),枚举类型持有固定集合的值，这些值被称为枚举的 **成员（variants）**  
`Result` 的成员是 `Ok` 和 `Err`，`Ok` 成员表示操作成功，内部包含成功时产生的值。`Err` 成员则意味着操作失败，并且包含失败的前因后果  
`Result` 类型的值，像其他类型一样，拥有定义于其上的方法。`io::Result` 的实例拥有`expect` 方法。如果 `io::Result` 实例的值是 `Err`，`expect` 会导致程序崩溃，并显示当做参数传递给 `expect` 的信息。如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。如果 `io::Result` 实例的值是 `Ok`，`expect` 会获取 Ok 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数

### 使用PrintIn!占位符打印值

```.rs
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
```

{}是占位符,第一个{}对应逗号后面第一个变量,第二个对应第二个变量

### 在使用库的时候对Cargo.toml文件的修改

假如要使用`rand`库,那么需要在`Cargo.toml`里面添加  

```.toml
rand="0.3.14"
```

它表示 “任何与 0.3.14 版本公有 API 相兼容的版本”。  
修改了之后,执行`cargo build` 可以直接从`cargo.io`上下载所依赖的`crate`

### Cargo.lock保证构建是可重现的

Cargo 有一个机制来确保任何人在任何时候重新构建代码，都会产生相同的结果  
比如:上面安装的`rand`包升级了`(0.3.15)`,如果升级里面包含会破坏代码运行的东西,那么在运行这个代码会发生什么呢?  
当有了`Cargo.lock`文件之后,就不用担心。当第一次构建项目时，Cargo 计算出所有符合要求的依赖版本并写入 `Cargo.lock` 文件。当将来构建项目时，Cargo 会发现 Cargo.lock 已存在并使用其中指定的版本，而不是再次计算所有的版本。这使得你拥有了一个自动化的可重现的构建。换句话说，项目会持续使用 `0.3.14` 直到你显式升级(自己改变toml文件里面的依赖包)，多亏有了 Cargo.lock 文件。

#### 更新一个crate到一个新的版本

当你需要把`rand`升级到**0.3.15**的时候,直接执行`cargo update`,这个命令不会考虑`cargo.lock`里面的内容,直接根据`cargo.toml`里面的包名称,升级相应的包(不过只能从0.3.14升级到0.3.15,不可以升级到0.4.1),如果你想要升级到0.4.1,那么必须更改cargo.toml里面的报名版本号:

```.toml
rand="0.4.1"
```

### 查看当前工程所用的crate

`cargo doc --open`

### std::cmp::Ordering

同 `Result` 一样， `Ordering` 也是一个枚举，不过它的成员是 `Less、Greater 和 Equal`。这是比较两个值时可能出现的三种结果。  

### match语句

感觉跟switch差不多的