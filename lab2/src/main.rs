// підключаю бібліотеки
use std::io;
use std::str::FromStr;

fn main() {

    // останній результат операції
    let mut last_result: Option<f64> = None;

    loop {
        // ввід користувача
        println!("Введіть перше число (або 'exit' для виходу):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка при зчитуванні рядка.");
        
        // вихід
        if input.trim().eq("exit") {
            println!("Дякуємо за користування калькулятором!");
            break;
        }

        // конвертую string в число
        let num1: f64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Будь ласка, введіть дійсне число!");
                continue;
            }
        };

        // оператор
        println!("Введіть оператор (+, -, *, /):");
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Помилка при зчитуванні рядка.");
        let operator = operator.trim();

        // друге число
        println!("Введіть друге число:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка при зчитуванні рядка.");
        
        // конвертую string в число
        let num2: f64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Будь ласка, введіть дійсне число!");
                continue;
            }
        };

        // обчислення
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Помилка: ділення на нуль!");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Невідомий оператор. Спробуйте ще раз.");
                continue;
            }
        };

        last_result = Some(result);

        println!("Результат: {}", result);
    }
}

