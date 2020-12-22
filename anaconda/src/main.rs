use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Bienvenue dans le jeu de l'Anaconda. Le but est de découvrir la taille de l'Anaconda avant qu'il ne vous étouffe. La seule chose que vous savez est que sa taille est comprise entre 4 et 31 et que cette taille change à chaque fois que vous relancez le jeu.");

    let secret_number = rand::thread_rng().gen_range(4, 31); // à cette étape, on choisit un chiffre au hasard pour avoir la taille de l'Anaconda

    loop {
        println!("Si vous voulez bien prendre la peine de proposer une taille pour mon anaconda (oui, j'ai oublié de préciser, c'est le mien)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Oupss, vous avez fait quoi ?! L'erreur ne vient jamais du programmeur, sachez le :-p");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Vous proposez : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Heu ... mon anaconda va se vexer °_O"),
            Ordering::Greater => println!("Hum, quand même pas. Il est grand, mais pas à ce point !"),
            Ordering::Equal => {
                println!("Ouf ! L'anaconda ne vous étouffera donc pas cette fois-ci. Ayez confiance, croyez en lui !");
                break;
            }
        }
    }
}