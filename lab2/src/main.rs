// Підключаємо необхідні бібліотеки
use std::io;
use std::str::FromStr;

// Оголошуємо головну функцію
fn main() {

    // Змінна для зберігання останнього результату операції
    let mut last_result: Option<f64> = None;

    loop {
        // Виводимо повідомлення та запитуємо користувача на вхід
        println!("Введіть перше число (або 'exit' для виходу):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка при зчитуванні рядка.");
        
        // Перевіряємо чи користувач хоче вийти
        if input.trim().eq("exit") {
            println!("Дякуємо за користування калькулятором!");
            break;
        }

        // Пробуємо перетворити вхід на число
        let num1: f64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Будь ласка, введіть дійсне число!");
                continue;
            }
        };

        // Запитуємо оператор
        println!("Введіть оператор (+, -, *, /):");
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Помилка при зчитуванні рядка.");
        let operator = operator.trim();

        // Запитуємо друге число
        println!("Введіть друге число:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка при зчитуванні рядка.");
        
        let num2: f64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Будь ласка, введіть дійсне число!");
                continue;
            }
        };

        // Виконуємо операцію
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

        // Зберігаємо результат
        last_result = Some(result);

        // Виводимо результат
        println!("Результат: {}", result);
    }
}

