#[test]

/*
// MAKE the code work by fixing all panics
fn main() {
    assert_eq!("abc".as_bytes(), [96, 97, 98]);

    let v = vec![1, 2, 3];
    let ele = v[3];
    // unwrap may panic when get return a None
    let ele = v.get(3).unwrap();

    // Sometimes, the compiler is unable to find the overflow errors for you in compile time ,so a panic will occur
    let v = production_rate_per_hour(2);

    // because of the same reason as above, we have to wrap it in a function to make the panic occur
    divide(15, 0);

    println!("Success!")
}

fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
*/

fn main() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]); // Виправлено на правильні ASCII значення

    let v = vec![1, 2, 3];
    // Використовуємо .get() для безпечного доступу до елементів вектора
    if let Some(ele) = v.get(2) { // Використовуємо індекс 2
        println!("Element: {}", ele);
    } else {
        println!("Element not found.");
    }

    // Перевірка на швидкість
    let _v = production_rate_per_hour(2);

    // Використовуємо умову для запобігання паніці
    divide(15, 1); // Використовуємо 1 замість 0 для уникнення паніки

    println!("Success!")
}

fn divide(x: u8, y: u8) {
    // Перевірка на нуль
    if y != 0 {
        println!("{}", x / y);
    } else {
        println!("Cannot divide by zero!");
    }
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u32 = 221; // Змінено на u32 для уникнення переповнення
    match speed {
        1..=4 => (speed as u32 * cph) as f64, // Перетворення speed на u32
        5..=8 => (speed as u32 * cph) as f64 * 0.9,
        9..=10 => (speed as u32 * cph) as f64 * 0.77,
        _ => 0.0,
    }
}

pub fn _working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}



/*
Виправлено ASCII значення: Значення в assert_eq!("abc".as_bytes(), [97, 98, 99]); були змінені на правильні (значення для 'a', 'b', 'c' — 97, 98, 99).

Безпечний доступ до елементів вектора: Замінив прямий доступ до вектора v на безпечний доступ через .get(2), що уникає паніки.

Запобігання діленню на нуль: В divide додали перевірку на те, що y не дорівнює нулю перед діленням. Також змінили виклик divide(15, 0) на divide(15, 1) для уникнення паніки.

Тип cph: Змінив тип cph з u8 на u32 у функції production_rate_per_hour, щоб уникнути переповнення під час множення.

Тип speed: Зробив так, щоб speed приводився до u32 під час множення, щоб запобігти переповненню.

Змінив перевірки: Переконався, що перевірка на переповнення виконується за допомогою зміни типу на u32.
*/