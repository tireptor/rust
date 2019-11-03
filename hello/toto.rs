use std::io;
use std::str::FromStr;

fn main() {
    println!("Hello world!");
    let a = 12;
    let b = 13;
    let c = a + b;
    let reponse : Option<isize>;
    let alphabet = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
    println!("La variable a est égale à : {}", a);
    println!("Le calcul de {} + {} est égale à {}",a,b,c );
    recuperer_entree_utilisateur();
    //reponse = recuperer_entree_utilisateur_numerique();
    
    //println!("{:?}",reponse);
    while true {

    }
}
fn recuperer_entree_utilisateur_numerique() -> Option<isize> { // elle ne prend rien en entrée et retourne un Option<isize> (dans le cas où ça ne fonctionnerait pas)
    let mut entree = String::new();

    match io::stdin().read_line(&mut entree) { // on récupère ce qu'a entré l'utilisateur dans la variable entree
        Ok(_) => { // tout s'est bien passé, on peut convertir la String en entier
            match isize::from_str(&entree.trim()) { // la méthode trim enlève tous les caractères "blancs" en début et fin de chaîne 
                Ok(nombre) => Some(nombre), // tout s'est bien déroulé, on retourne donc le nombre
                Err(_) => { // si jamais la conversion échoue (si l'utilisateur n'a pas rentré un nombre valide), on retourne None
                    println!("Veuillez entrer un nombre valide !");
                    None
                }
            }
        },
        _ => { // une erreur s'est produite, on doit avertir l'utilisateur !
            println!("Erreur lors de la récupération de la saisie...");
            None
        }
    }
}
fn recuperer_entree_utilisateur() { // elle ne prend rien en entrée et retourne un Option<isize> (dans le cas où ça ne fonctionnerait pas)
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
    Ok(n) => {
        println!("{} bytes read", n);
        println!("{}", input);
    }
    Err(error) => println!("error: {}", error),
}
}