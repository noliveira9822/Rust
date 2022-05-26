/*
Convert temperatures between Fahrenheit and Celsius.
Celsius = (T * 1.8) + 32 F
Fahrenheit = (T - 32) / 1.8
*/

use std::io;

fn main() {
    let dif: f32 = 32.0;
    loop {
        let mut choice = String::new();
        println!("*************************************************");
        println!("*      Escolha uma unidade de temperatura.      *");
        println!("*************************************************");
        println!("*                  0 - Celsius                  *");
        println!("*                1 - Fahrenheit                 *");
        println!("*                                               *");
        println!("*                   9 - Exit                    *");
        println!("*************************************************");

        io::stdin().read_line(&mut choice).expect("Falha ao ler o input.");

        let escolha: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match escolha {
            0 => {
                println!("*************************************************");
                println!("*   Indique o valor da temperatura em Celsius:  *");
                let mut temperatura = String::new();
                io::stdin().read_line(&mut temperatura).expect("Erro ao ler o valor introduzido.");
                let temperatura: f32 = match temperatura.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temperatura_fahrenheit = (temperatura * 1.8) + dif;
                println!("*     A temperatura em Fahrenheit é: {:.2}.    *", temperatura_fahrenheit);
                println!("*************************************************");
            }
            1 => {
                println!("*************************************************");
                println!("* Indique o valor da temperatura em Fahrenheit: *");
                let mut temperatura = String::new();
                io::stdin().read_line(&mut temperatura).expect("Erro ao ler o valor introduzido.");
                let temperatura: f32 = match temperatura.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temperatura_celsius = (temperatura - dif) / 1.8;
                println!("*      A temperatura em Celsius é: {:.2}        *", temperatura_celsius);
                println!("*************************************************");
            }
            9 => {
                println!("*************************************************");
                println!("*                     Xau aí...!                *");
                println!("*************************************************");
                break;
            }
            _ => {}
        }
    }
}
