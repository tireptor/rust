use std::fs::File;
fn main() {

    let mut fichier = match File::open("toto.txt") {
        Ok(f) => {
            // Okay, l'ouverture du fichier s'est bien déroulée, on renvoie l'objet
            f
        },
        Err(e) => {
            // Il y a eu un problème, affichons l'erreur pour voir ce qu'il se passe
            println!("{}", e);
            // on ne peut pas renvoyer le fichier ici, donc on quitte la fonction
            return;
        }
    };
    loop {

    }

}
