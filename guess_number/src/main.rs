use std::io; //引用标准库里面的io库,为了获取用户输入并打印结果作为输出,
             //需要将 io（输入/输出）库引入当前作用域

//相当于use rand 表示使用外面的库
//所以现在可以使用 rand:: 前缀来调用 rand crate 中的任何内容。
extern crate rand;
//Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    //程序的入口
    println!("please guess the number"); //两个宏定义打印字符串

    //rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，
    //并从操作系统获取 seed。接下来，调用随机数生成器的 gen_range 方法。
    //这个方法由刚才引入到作用域的 Rng trait 定义。gen_range 方法获取两个数字作为参数，
    //并生成一个范围在两者之间的随机数
    let _secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input your guess");

        //let语句创建一个新变量,mut表示该变量是可变的,guess就是变量名
        //String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块。
        //String::new 的结果，这个函数会返回一个 String 的新实例
        //::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数
        //new 函数创建了一个新的空字符串，它是创建类型实例的惯用函数名。
        let mut guess = String::new();

        //调用stdin里面的read_line函数,读取用户的输入,无论输入什么,都将其存入到一个字符串中
        //这个字符串就是guess,要求其可变,就必须加上mut参数
        //& 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //将输入的字符串转化为u32类型的数字,rust默认的数字类型是u32
        //在这里其实初始化了另外一个变量guess,它会覆盖之前的变量(shadow)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //如果guess.trim().parse()返回的Result为true,那么就取得OK里面的数字
            Err(_) => {
                println!("please input a number");
                continue;
            } //如果转化数字失败,那么就直接执行下一次循环
        };

        //这行代码打印储存到guess里面的字符串.{}是占位符
        println!("You guess,{}", guess);

        //match语句,其实跟switch差不多,guess.cmp()返回的是一个Ordering枚举类型
        //跟下面三个语句进行比较,看符合哪一个,就执行后面的语句
        match guess.cmp(&_secret_number) {
            Ordering::Greater => println!("Too bigger"),
            Ordering::Equal => {
                println!("you are right");
                break; //退出循环
            }
            Ordering::Less => println!(" Too less"),
        }
    }
}

// "rust.rustup": {

//     "nightlyToolchain": "nightly-x86_64-unknown-linux-gnu"
// },
// "rust.mode": "rls",
// "rust.rls": {
//     "useRustfmt": true
// },
