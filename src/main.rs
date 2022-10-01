use std::io;

/*
TODO: вынести формулу 24 (x - 10sinX + |x^4 - x^5|) в отдельную функцию, сделать мейн лупом 
*/


fn main() {
    let mut x = String::new();
    println!("Введите значение Х:");

    io::stdin()
        .read_line(&mut x)
        .expect("Ошибка чтения ввода");
        let mut x: f64 = x.trim().parse().expect("Ввод должен быть числовой");
    
}
