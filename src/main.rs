use std::io;

/*
TODO: вынести формулу 24 (x - 10sinX + |x^4 - x^5|) в отдельную функцию, сделать мейн лупом
Функции для задачи: .abs(), .sin(), .pow()
*/

//Функция main служит для считывания ввода и коммуникации с пользователем (фронтенд)
fn main() {
    loop {
        let mut x = String::new(); //создание переменной Х
        println!("Введите значение Х:");

        io::stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную Х

        let mut x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная Х должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки

        let mut y: f64 = calc(x);
        println!("{y}")
    }
}

fn calc(x: f64) -> f64 {
let y: f64 = x.powi(4);
y
}