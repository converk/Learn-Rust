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
