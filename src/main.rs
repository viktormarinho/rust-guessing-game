use rand::{thread_rng, Rng};
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
    let random_number = thread_rng().gen_range(1..=100);
    let mut tentativas = 0;
    println!("Tente adivinhar o número que sorteei entre 1 e 100!");

    let correct_guess = loop {
        tentativas += 1;
        let guess = input("Digite um número:")
            .parse::<i32>()
            .expect("Erro: não foi digitado um número");

        if guess > random_number {
            println!("Muito alto!");
        } else if guess < random_number {
            println!("Muito baixo!");
        } else {
            println!("Parabéns! você acertou!");
            break guess;
        }
    };

    println!(
        "A resposta correta era {}. Você precisou de {} tentativas.",
        correct_guess, tentativas
    );
}
