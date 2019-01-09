use std::io; //引用标准库里面的io库,为了获取用户输入并打印结果作为输出,
             //需要将 io（输入/输出）库引入当前作用域

fn main() {
    //程序的入口
    println!("please guess the number"); //两个宏定义打印字符串
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
}
