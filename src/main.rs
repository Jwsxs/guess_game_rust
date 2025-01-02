use std::io; //using std::input output
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new(); //cria um novo "guess" que vai receber um valor
        io::stdin()
            .read_line(&mut guess) //lê linha e assume como valor de guess
            .expect("Failed to read."); //crasha aoqualquer erro

        let guess: u32 = match guess //guess que já tem lá em cima, com match pra erro
            .trim() //elimina qualquer espaço branco
            .parse() {
                Ok(num) => num, //caso número, vai
                Err(_) => {
                    println!("Should be a number!\n");
                    continue;
                }, //se não, continua até ir
            };//converte string em outro, nesse caso em u32 (unsigned 32-bit)

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
