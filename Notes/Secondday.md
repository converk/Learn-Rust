# Rust学习笔记--第2天

## 变量与可变性

Rust里面变量默认是不可变的,即如果有以下代码

```.rs
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

那么就会报错说`cannot assign twice to immutable variable`, 不可以对变量二次赋值, 可以通过使用`mut`来使得变量可变  

```.rs
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

### 变量与常量

对于常量而言,不是默认不可变,而是一直都不可变,也不能使用`mut`关键字,  
常量使用关键字`const`来定义,**而且在声明的时候一定要标明常量的类型**,常量可以在任意作用域中声明

```.rs
const MAXSIZE: u32 = 10000;
```

常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他**只能在运行时计算出的值**。  
在声明它的作用域之中，常量在整个程序生命周期中都有效

### 隐藏

我们可以定义一个与之前变量同名的新变量，而新变量会 **隐藏** 之前的变量,这意味着使用这个变量时会看到第二个值。可以用相同变量名称来隐藏一个变量，以及重复使用 `let` 关键字来多次隐藏  

```.rs
fn main(){
    let x=5;
    let x=x+1;
    let x=x*x;
    println!("x: {}", x);
}
```

#### 隐藏与mut的区别

看起来二者都可以改变变量的值,

1. 但是对于**隐藏**来说改变之后x的值依然是不可变的,对于`mut`来说,依旧是可变的
2. 而且使用隐藏可以改变一个变量的类型,因为使用了`let`,相当于重新定义了一个名字相同的变量

    ```.rs
    let spaces="    ";
    let spaces=spaces.len();
    ```

    第一个`spaces`的类型是字符串,第二个的类型是整形  
    但是对于`mut`来说,当一个变量定义之后类型就确定了,就算声明了`mut`也不可改变类型

    ```.rs
    let mut spaces = "   ";
    spaces = spaces.len();
    ```

---

## 数据类型

两大数据类型集合:`标量`和`复合`  
Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。  

### 标量

标量有四种:**整形,浮点型,布尔型,字符串**

#### 整形

长度(bit) | 有符号 | 无符号
---------|----------|---------
 8 | i8 | u8
 16 | i16 | u16
 32 | i32 | u32
 64 | i64 | u64
 arch | isize | usize

每一个有符号的变体可以储存包含从 **-(2^n - 1)** 到 **2^(n - 1) - 1** 在内的数字，这里 n 是变体使用的位数。所以 i8 可以储存从 -(2^7) 到 2^7 - 1 在内的数字，也就是从 -128 到 127。无符号的变体可以储存从 0 到 `2^n - 1` 的数字，所以 u8 可以储存从 0 到 `2^8 - 1` 的数字，也就是从 0 到 255。  
 `isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的  
 数字类型默认是 `i32`：它通常是最快的，甚至在 64 位系统上也是

##### 溢出

如果一个数的类型是u8,那么它最多存储255,但如果你把256赋给它,那么就称为**溢出**

#### 浮点数

Rust里面有两个浮点数类型,`f32`和`f64`,**默认类型是f64**  
`f32` 是单精度浮点数，`f64` 是双精度浮点数  
声明一个浮点类型,可以指定其类型(通过使用`:`)

```.rs
let x: f32=3.1;
```

#### 布尔值

也是有`true`和`false`两种,一般用于流程控制

```.rs
fn main() {
    let t = true;
    let f: bool = false; // 显式指定类型注解
}
```

#### 字符类型

Rust 也支持字母。Rust 的 `char` 类型是语言中最原生的字母类型，如下代码展示了如何使用它。（注意 char 由**单引号**指定，不同于**字符串使用双引号**。）  
Rust 的 char 类型代表了一个 `Unicode` 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，**拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符**都是有效的 `char` 值

### 复合

顾名思义,就是把几种类型组合在一起
只有两种类型  `元组`和`数组`

#### 元组

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的

```.rs
let tup: (i32,f64,u8)=(32,6.4,8);  //根据每个位置对应的类型来进行赋值
```

对于元组类型,也必须要加上mut才能改变他里面元素的值,

```.rs
let mut tup=(64.32.1);
tup.0=5;
```

变量tup被绑定到整个元祖上,因为tup是一个复合类型,  
如果我们需要从元组里面获取单个元素,可以**使用模式匹配来解构**,也可以使用点号 **(.)** 后跟值的索引来直接访问它们

1. 模式匹配

   ```.rs
   fn main(){
       let tup=(32,6.4,8);
       let (x,y,z)=tup;
       println!("y:{}",y);
   }
   ```

2. 索引

    ```.rs
    fn main(){
       let tup=(32,6.4,8);
       let x=tup.0;
       println!("y:{}",x);
   }
    ```

#### 数组

与在C语言里面一样,**数组的每个成员的数据类型都必须相同**,并且数组的长度一旦声明就不可改变,也就是静态数组  
声明一个数组

```.rs
fn main(){
    let a=[1,2,3,4,5];

    //可以直接声明数组里面的元素类型和元素个数,也可以省略
    //两个之间使用分号分割
    let b:[i32; 5]=[1,2,3,4,5];  
}
```

访问数组和无效的索引

```.rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

如果一个索引是无效的:

```.rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    //Rust会对索引进行检查,一旦发现索引超过长度,那么就会退出
    //不像C语言那样,会访问到外界的数据
    let first = a[10];  
    println!("{}",first);
}
```

## 函数

一个函数的定义在引用这个函数前或者后都行,只要定义了就行

### 函数的参数

给函数传递参数的时候,必须要声明每个参数的类型,当一个函数有多个参数的时候,使用逗号分隔开,  
而且要求给函数传递参数的时候,类型一定要对,不存在形参的类型是整形,传进去实参是浮点型,编译器不会进行强制类型转换,而是会跑出错误,程序无法运行,同样,形参是浮点型,实参是整形也不行

```.rs
fn main() {
    another_fn(5, 6.1);
    another_fn(5, 7.1);
}
fn another_fn(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

### 包含语句和表达式的函数体(有点重要,理解一下)

函数体由一系列的**语句**和一个可选的结尾**表达式**构成  
Rust 是一门**基于表达式**（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别  
**语句**: 语句是执行一些操作但**不返回值**的指令  
**表达式**: 表达式计算并产生一个值  

#### 语句

```.rs
fn main() {
    let y = 6;  //这是一个语句而不是一个表达式,因为它没有返回值
}
```

上面的整个**main**函数就是一个语句,没有返回值,  
由于`let y=6`没有返回值,那么`let x = (let y = 6);`就是不对的

#### 表达式

表达式会计算出一些值，并且你将编写的大部分 Rust 代码是由表达式组成的。考虑一个简单的数学运算，比如 `5 + 6`，这是一个表达式并计算出值 11。语句 `let y = 6`; 中的 `6` 是一个表达式，它计算出的值是 6。函数调用是一个表达式。宏调用是一个表达式。我们用来创建**新作用域**的大括号（代码块），`{}`，也是一个表达式.一个例子:

```.rs
fn main() {
    let x = 3;
    let y = {  //这个大括号里面是一个新的作用域里面的x不会对外面的x造成影响
        let x = 3;
        x + 1
    };
    println!("x: {}", x);
    println!("y: {}", y);
}
```

```.rs
{
    let x = 3;
    x + 1
}
```

是一个代码块，它的值是 4。这个值作为 `let` 语句的一部分被绑定到 `y` 上。注意结尾没有分号的那一行 `x+1`，与你见过的大部分代码行不同。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值

#### 具有返回值的函数

函数可以向调用它的代码返回值。我们并不对返回值命名，但要在**箭头（->）**后声明它的类型。在 Rust 中，函数的返回值等同于函数体**最后一个表达式**(表达式不带分号)的值。使用 `return` 关键字和指定值，可从函数中提前返回

```.rs
fn main() {
    ("x: {}", another_fn() + 1);
    println!("y: {}", another_fn() + 2);
}
fn another_fn() -> u32 {
    5
}
```

`another_fn()`里面的5就是一个表达式,作为返回值返回,但是如果在5后面加一个`;`就变成一个语句,就不是返回值了,如果这时候运行,会报错,提示删除;  
也可以带参数进行返回  

```.rs
fn main() {
    println!("x: {}", another_fn(5) + 1);
    println!("y: {}", another_fn(7) + 2);
}
fn another_fn(x: u32) -> u32 {
    x + 1
}
```

## 控制流

Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环。

### if

要注意的是`if`和`else if`里面的判断值必须是`bool`值,如果是`int` 或者是其他则编译器会报错,比如下面这个例子

```.rs
fn main() {
    let x = 3;
    if x {          //这里的x是int型,所以编译器会报错
        println!("x: {}", another_fn(5) + 1);   //这是一条语句,不是表达式
    } else {
        println!("y: {}", another_fn(7) + 2);
    }
}
```

#### 在let语句里面使用if

```.rs
fn main() {
    let condition = true;
    let x = if condition { 5 } else { 6 };  //每一个大括号里面都是一个代码块,代码块的值是最后一个表达式的值
    println!("x:,{}", x);
}
```

整个 `if` 表达式的值取决于哪个代码块被执行。这意味着 `if` 的每个分支的可能的返回值都必须是相同类型；`if` 分支和 `else` 分支的结果都是 `i32` 整型。如果它们的类型不匹配，如下面这个例子，则会出现一个错误：

```.rs
fn main() {
    let condition = true;
    let x = if condition { 5 } else { "sx" };
    println!("x:,{}", x);
}
```

下面这种也是不行的

```.rs
fn main() {
    let condition = true;
    let x: i64 = if condition { 5.0 } else { 6.0 };
    println!("x:,{}", x);
}
```

Rust 需要在编译时就确切的知道 `x` 变量的类型，这样它就可以在编译时验证在每处使用的 `x` 变量的类型是有效的。

### 循环

#### loop

`loop` 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止

```.rs
fn main() {
    loop {
        println!("again!");
    }
}
```

#### 从循环中返回

`loop` 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。然而你可能会需要将操作的结果传递给其它的代码。如果将返回值加入你用来停止循环的 `break` 表达式，**它会被停止的循环返回**：

```.rs
fn main() {
    let mut counter = 0;
    let x = loop {
        counter += 1;
        if counter == 10 {
            break 2 * counter; //作为返回值返回到x,break表达式,最后有一个分号
        }
    }; //不要忘了分号
}
```

注意作用域的问题

```.rs
fn main() {
    let counter = 0;
    let x = loop {   //这里的loop每执行完一次,counter的值不会变成上一次循环是时counter的值,而是会变成0
        let counter = counter + 1;
        println!("counter,{}", counter);
        if counter == 10 {
            break 2 * counter; //作为返回值返回到x,break表达式,最后有一个分号
        }
    }; //不要忘了分号
    println!("x,{}", x);
}
```

#### while循环

跟其他语言差不多,就不解释了

#### for循环

```.rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element,{}", element);
    }
}
```

这里不需要考虑数组越界的问题,但是如果是用`while`循环的话就需要写出退出条件
`for` 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。即使是在想要循环执行代码特定次数时，例如示例 3-3 中使用 while 循环的倒计时例子，大部分 Rustacean 也会使用 for 循环。这么做的方式是使用 `Range`，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。

下面是一个使用 `for` 循环来倒计时的例子，它还使用了一个我们还未讲到的方法，`rev`，用来反转

```.rs
fn main() {
    for element in (1..4).rev() {   //(1..4)是一个range,包含了1,2,3三个元素,rev()是将这三个元素倒过来
        println!("element,{}", element);
    }
}

```

## 练习

### 斐波那契数列

```.rs
use std::io;
fn main() {
    let mut fb_num = String::new();
    io::stdin().read_line(&mut fb_num).expect("read error");
    let fb_num = fb_num.trim().parse().expect("please input anumber");
    println!("result,{}", fb(fb_num));
}

fn fb(x: u32) -> u32 {
    if x == 1 || x == 0 {
        1
    } else {
        fb(x - 1) + fb(x - 2)
    }
}

```

### 转换华氏与摄氏

```.rs
use std::io;
fn main() {
    println!("please input:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read error ");
    let (first, last) = input.split_at(input.len() - 2);
    let input_number: f32 = first.trim().parse().expect("please input a number");
    if last == "c\n" || last == "C\n" {
        println!("华氏,{}F", (input_number * 1.8) + 32.0);
    } else {
        println!("摄氏,{}C", (input_number - 32.0) / 1.8)
    }
}
```