//! 解析流程：
//! 1、我们应该判断是否输入了终止条件；有的话，直接返回；
//! 2、逐字节读取，判断token，直到结尾；
//! 3、将token转换为node，同时应用运算符优先级规则
//! 4、输出结果。
use std::io::Stdin;

fn main() {
    let mut buf = String::new();
    let std_in: Stdin = std::io::stdin();
    while std_in.read_line(&mut buf).is_ok() {
        pure_buf = buf.trim_end();
        // if receive stop signal, stop it.
        if "bye" == pure_buf {
            println!("bye");
            break;
        }

        // change buf as bytes, so we can convert it into tokens.
        let text = pure_buf.trim_end();
        // let c: char = match text.chars().next() {
        //     Some(next_char) => next_char,
        //     None => {
        //         return;
        //         // return Ok((Tok::EOF, 0));
        //     }
        // };
        // let () =  match c {
        //
        // }
        println!("{:?}", text);
        buf.clear();
    }
}
