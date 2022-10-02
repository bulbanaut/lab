use std::io::stdin;

/*
TODO: -
Функции для задачи: .abs(), .sin(), .pow()
*/

//Функция main служит для считывания ввода и коммуникации с пользователем (фронтенд)
fn main() {
    loop { //инициализация петли
        let mut x = String::new(); //создание переменной X
        println!("Введите значение Х:"); //вывод сообщения с просьбой ввести значение X

        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная Х должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки

        let x: f64 = calc(x); //переменная Y приравнивается к функции calc от X
        println!("Результат {x}");
        pause(); //использование фунции паузы
        break; //разрыв петли
    }
}

fn calc(x: f64) -> f64 {
let x: f64 = (&x - 10.0*&x.sin()) + (&x.powi(4) - &x.powi(5)).abs(); //функция calc выполняет y(x) = x - 10sinX + |x^4 - x^5|
x
}

fn pause() { //фукция паузы
    let mut q = String::new();
    println!("Нажмите Enter чтобы выйти.");
    stdin().read_line(&mut q).expect("ошибка");
}