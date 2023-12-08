// package(cargo toml) -> crate -> mod
// crate根文件为src/main.rs或src/lib.rs
// 在crate根文件中声明模块后，比如mod gardon，编译器会到如下位置寻找模块的定义：
// 1.内联，mod gardon后的{}内
// 2.src/gardon.rs
// 3.src/gardon/mod.rs  (老风格，不推荐)
// 声明子模块，在非crate根文件中定义的模块，都是子模块。比如mod vegetables
// 1.内联
// 2.src/gardon/vegetables.rs
// 3.src/gardon/vegetables/mod.rs  (老风格，不推荐)
mod authentication;

use basic_concept;
use std::io;

fn study_io() {
    let age = 30;
    println!("guess the number, {} {age}", "yao");
    println!("please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

fn study_module() {
    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    user.set_password("even-more-secret");
}

fn main() {
    // todo!("print this message on compile");
    basic_concept::study_basic_concept();

    study_module();
}
