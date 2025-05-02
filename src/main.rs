use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("=== Jeu de devinette de nombre ===");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    println!("J'ai choisi un nombre entre 1 et 100.");
    println!("À vous de le deviner !");

    let mut nb_essais = 0;

    loop {
        print!("Votre proposition : ");
        io::stdout().flush().unwrap();

        // lire l'entrée
        let mut proposition = String::new();
        io::stdin()
            .read_line(&mut proposition)
            .expect("Échec de la lecture de l'entrée");

        // convertir l'entrée en nombre
        let proposition: u32 = match proposition.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        nb_essais += 1;

        // comparer
        match proposition.cmp(&nombre_secret) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Gagné ! Le nombre était bien {}.", nombre_secret);
                println!("Vous avez trouvé en {} essais.", nb_essais);
                break;
            }
        }
    }
}
