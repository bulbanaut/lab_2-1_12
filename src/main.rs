use std::io::stdin;

fn main() {
    let b = String::from("Мальчик");
    let g = String::from("Девочка");

    println!("Кто ты: Мальчик или Девочка?");
    let input = read_var();
    if input == g {
        println!("Мне нравятся мальчики");
    } else if input == b {
        println!("Мне нравятся девочки");
    } else {
        println!("Ввод не распознан");
    }
}

fn read_var() -> String {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)
        let x = x.trim().to_string();
        return x;
    }
}
