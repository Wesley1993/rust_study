use std::io;

fn main() {
    println!("猜数!");

    println!("猜一个数！");

    // let mut foo = 1;
    // let bar = foo;

    // foo = 2;

    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("无法读取数");

    println!("你猜测的数字为：{}",guess);
}
