use std::io;

fn input(message: &str) -> String {
    let mut user_input = String::new();

    println!("{message}");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Erro ao ler o input do usuário");

    user_input.trim().to_string()
}

fn main() {
    let number = input("Digite um número:")
        .parse::<i32>()
        .expect("Erro: não foi digitado um número");

    println!("{number}");
}
