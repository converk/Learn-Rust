# Rust学习笔记--所有权系统(owership)

其令 **Rust** 无需垃圾回收（garbage collector）即可保障内存安全

## 什么是所有权

### 堆与栈

对于rust,编译时数据的存放位置是很重要的,  

1. 对于栈的操作是十分快速的,因为栈的存储数据的位置总是在栈顶,并且,栈里面存储的数据必须是大小已知的,并且固定的数据
2. 在编译时大小未知或大小可能变化的数据，要改为存储在堆上。**堆是缺乏组织的**：当向堆放入数据时，你要请求一定大小的空间。操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 **在堆上分配内存**

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。

### 所有权规则

1. Rust里面每一个值都有一个被称为其**所有者**的变量
2. 值有且只有一个所有者
3. 当所有者(变量)离开作用域之后,这个值会被丢弃

### 变量作用域

```.rs
#![allow(unused_variables)]
fn main() {
{                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的

    // 使用 s
}                      // 此作用域已结束，s 不再有效
}
```

这个作用域和其他语言好像是类似的

### String类型

这个类型比之前讲的那几种标量和复合类型都要复杂,这个类型的数据是存放在堆上而不是栈,以此来说明rust是如何知道在何时清理数据的  
对于上例中的`let s = "hello";`里面的`"hello"`是一个字符串字面值,不可改变,其数据存放在栈上,因此这样也就造成了一个缺点.对于如果想要读取用户的输入就不能使用这种类型.  
因此,`String`类型就解决了这个问题,这个类型被分配到`堆`上，所以能够存储在编译时未知大小的文本。可以使用 from 函数基于字符串字面值来创建 `String`，如下：

```.rs
let s=String::from("hello");  
```

这句话把s初始化为值为`"hello"`的`String`类型,并且这个字符串s的值是可以修改的:

```.rs

let mut s = String::from("hello");   //要修改,必须命名时加上mut关键字

s.push_str(", world!"); // push_str() 在字符串后追加字面值

println!("{}", s); // 将打印 `hello, world!`
```

### String类型与字符串字面值在内存分配上的区别

1. 就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效
2. 对于 `String` 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：

    > 1. 必须在运行时向操作系统请求内存。
    > 2. 需要一个当我们处理完 String 时将内存返回给操作系统的方法。

    对于将内存返回给操作系统的方法,Rust采用了一个不同的策略:内存在拥有它的变量离开作用域后就被自动释放。

    ```.rs
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    }                                  // 此作用域已结束，
                                    // s 不再有效
    ```
    这是一个将 `String` 需要的内存返回给操作系统的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 `drop`，在这里 `String` 的作者可以放置释放内存的代码。Rust 在结尾的 `}` 处自动调用 `drop。`

### 变量与数据的交互方式(移动)

来观察两行代码

```.rs
let s1=String::from("hello");
let s2=s1;
```

首先第一行是初始化一个String类型的值,第二行将s1的值"拷贝到"s2里面,但是这里要注意,String类型的值是储存在堆上的,如果第二行再在堆上面拷贝相同的数据就没有任何意义,会影响到程序访问数据的速度,所以对于Rust来说,第二行并不是一个拷贝数据的操作,而是将s2的指针直接指向s1的数据在堆上的位置,但是如果只是这样的话,那么在s1和s2的作用域结束之后就会释放两次一模一样的堆空间,这样做显然是错的,因此,rust规定,如果对于存放在堆上面的数据进行像上述`let s2=s1`这样的操作之后,那么第一个变量`s1`就变无效,在其(s1)的作用域结束之后也不会对其进行drop操作,只会对s2进行drop来释放在堆里面的空间,也就是说,上面的代码,实际上把s1的值移动到了s2上面  
对于String的存储方式:指针存储在栈里面,二指针指向的数据存储在堆里面,栈里面还有长度和最大长度等数据

### 变量与数据的交互方式(克隆)

对于上面的移动操作,如果想继续保持s1有效,那么第二行代码可以变为

```.rs
let s2=s1.clone()
```

但是这样会消耗资源,而且运行速度也会变慢

### 只在栈上的数据--拷贝

如果一个变量的数据存放在栈上的话,那么进行下面的操作:

```.rs
let x=5;
let y=x;
```

就不会删除x,也就是在`let y=x`之后,x依然有效

### 所有权与函数

```.rs
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("{}", x);
    println!("{}", s);  //这里会报错

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

```

### 返回值与作用域

```.rs
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

```

有时候会考虑,当我们传入一个变量,就丢掉了它的所有权,但是如果我们接下来还要继续使用这个变量呢?,那么我们必须让这个变量从我们调用的函数中返回,但是这样难免有点麻烦,所以就有了**引用**,让一个函数只使用这个变量,但不获得其所有权

## 引用与借用

### 引用

引用的符号是`&`,就是C语言里面的取地址,在rust里面的含义也差不多是取地址,

```.rs
fn main() {
    let s1 = String::from("hello");

    //&s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
    //因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  //这里的s是对String的引用
    s.len()
}   // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
    // 所以什么也不会发生
```

事实上,在上述代码中,s是直接指向s1的存储地址的,相当于一个指向s1的指针,但是s并不能改变s1的值,只是可以使用s1的值,在函数`calculate_length`结束之后,s也就离开了作用域,但是它并不拥有引用值的所有权,所以,不会对s进行任何操作,下面来看一下对引用值进行改变会怎么样:

```.rs
fn main() {
    let mut s1 = String::from("hello");
    change_string(&s1);
}

fn change_string(s: &String) {
    s.push_str("world");
}
```

我们将获取引用作为函数参数称为 **借用**  
这个时候编译器会提示,`cannot borrow as mutable`,也就是不可改变借用变量的值

### 可变引用

可以通过将**引用**和**变量**都声明为可变的(mut),这样引用的值就可以修改原来的值了

```.rs
fn main() {
    let mut s1 = String::from("hello");
    change_string(&mut s1);
    println!("{}", s1);
}

fn change_string(s: &mut String) {
    s.push_str("world");
}

```

不过可变引用有一个很大的限制：**在特定作用域中的特定数据有且只有一个可变引用**。这些代码会失败:

```.rs
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

```

因为这样可能会导致多个指针同时修改同一块内存中的数据,可能会造成数据竞争,会造成多个指针的数据不同步(类似于git里面的冲突),但是我们可以使用一个大括号来声明一个新的作用域,在这个作用域里可以再次进行可变引用:

```.rs
fn main() {
    let mut s1 = String::from("hello");
    {
        let s2 = &mut s1;
        change_string(s2);
        println!("{}", s2);
    }
    println!("{}", s1);
    let s3 = &mut s1;
    change_string_2(s3);
    println!("{}", s3);
}

fn change_string(s: &mut String) {
    s.push_str("world");
}

fn change_string_2(s: &mut String) {
    s.push_str("world_1");
}

```

打印输出为

```.rs
helloworld
helloworld
helloworldworld_1
```

我们也不可以同时拥有**可变引用**和**不可变引用**(即使不在同一个作用域里面),因为不可变引用可不想在他们不知情的情况下,指向的值发生了改变,
但是我们可以同时拥有两个不可变引用,因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。

```.rs
fn main() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;  //错误
    let s3 = &s1;
    println!("{},{}", s3, s2);
}
```

```.rs
fn main() {
    let mut s1 = String::from("hello");
    let s2 = &s1;   //可以
    let s3 = &s1;
    println!("{},{}", s3, s2);
}
```

```.rs
fn main() {
    let mut s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    s1.push_str(",world"); //不可以,在有不可变引用的时候,即使原变量是可变的,那么也不可以改变
    println!("{},{},{}", s1, s3, s2);
}
```

```.rs
fn main() {
    let mut s1 = String::from("hello");
    let s2 = &s1;  //可变引用与不可变引用不在同一个作用域,但是还会报错
    {
        let s3 = &mut s1;
        change_string(s3);
    }

    println!("{},{}", s1, s2);
}
```

总而言之,在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。

## slice

### 字符串slice

字符串 `slice（string slice）`是 `String` 中一部分值的引用，它看起来像这样：

```.rs

#![allow(unused_variables)]
fn main() {
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
}
```

&s[0..5]的类型就是字符串slice,如果要返回这一类型,那么返回值就是`&str`,注意,slice所取的索引必须是在有效的范围之内,否则就会报错,注意:`&str`也就是字符串slice是`String`的**不可变引用**  
之前说字符串字面值是不可变的,因为**字符串字面值**的本质类型就是`&str`,即字符串slice,  
`let s="hello, world"`,s就是一个指向二进制程序特定位置的slice;

> [0..5]语法:相当于range(0,5),不包含5,如果想要包含5,那么就`[0..=5]`

下面这段代码展示区别:

```.rs
fn main() {
    let mut input = String::from("hello world");

    let input1 = &input[..]; //这里input1的类型是&str
    let input2 = &input1[..]; //input2的类型也是&str
    println!("{}", input2);
    println!("{}", input1 == input2); //结果为true
    println!("{},{}", input1 == input, input2 == input); //两个也全是true
    println!("{}", &input == &input[..]); //这个也是true

    let word = first_world(&input[..]);

    let input_strs = "hello world";
    let output = first_world(input_strs); //可以直接使用字符串字面量作为参数,因为其本质就是&str

    //input.clear(); //这里会提示,由于有input的不可变引用存在,那么不可以改变input的值
    println!("{}", word);
    println!("{}", output);
}

fn first_world(some_words: &str) -> &str {
    //接受不可变引用,返回不可变引用,
    //接受的和返回的全都是对input的不可变引用
    let bytes = some_words.as_bytes(); //先转换成byte

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &some_words[..i];
        }
    }
    &some_words[..]
}

```