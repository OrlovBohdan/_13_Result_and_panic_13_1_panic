#[test]

/*

// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        __
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink(__);

    println!("Exercise Failed if printing out this line!");
}
*/

fn main() {
    drink("lemonade"); // Заповнено

    println!("Exercise Failed if printing out this line!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        return; // Завершити виконання функції, якщо напій — "lemonade"
    }

    println!("Exercise Failed if printing out this line!");
}




/*
Перший пропуск: drink("lemonade"); — передаємо рядок "lemonade" у функцію drink, щоб виконати умову.

Другий пропуск: return; — завершує виконання функції, якщо напій — "lemonade", що запобігає виводу повідомлення про невдачу.
*/