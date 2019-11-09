
fn main() {

    //exercice1();
    //exercice2();
    //exercice3();
    //exercice4();
    exercice5 ();
    loop {

    }

}
fn exercice1 () {
    println!("Hello world!");
    let _i = 0i32;
    let tab = [0, 1, 2];
    let s = &tab;
    //tab[0] = 5;
    println!("{:?}",tab);
    println!("{:?}", s);
    let s = &tab[1..];
    println!("{:?}",s);
    let mut v : Vec<u8> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    let t = &v;
    println!("{:?}", t); // ça affichera "[0, 1, 2]"
    let t = &v[1..];
    println!("{:?}", t); // ça affichera "[1,2]
}

fn exercice2 () {
    let my_string = "hello";
    let my_secondString = "camping";
    let mut resultat : String ;
    let isOk = true;
    let age : i32 = 16;
    let i = 0i32;
    let une_variable = "jambon";
    let mut v = vec!(1, 2, 3);

    match my_string {
        "bonjour" => {
            println!("francais");
        }
        "ciao" => {
            println!("italien");
        }
        "hello" => {
            println!("anglais");
        }
        "hola" => {
            println!("espagnol");
        }
        _ => {
            println!("je ne connais pas cette langue...");
        }
    }
    match my_secondString {
        "hotel" => {
            resultat = String::from("Vous préférez l'hotel !");
        }
        "camping" => {
            resultat = String::from("Vous préférez le camping !");
        }
        "location particulier" => {
            resultat = String::from("Vous préférez une location chez un particulier !");
        }
        _ => {
            resultat = String::from("Vous n'avez aucune préférence !");
        }
    }
    println!("{}",resultat);
    // Récupération du résultat directement dans le switch
    let resultat2 = match my_secondString {
        "hotel" => "Vous préférez l'hotel !",
        "camping" => "Vous préférez le camping !",
        "location particulier" => "Vous préférez une location chez un particulier !",
        _ => "Vous n'avez aucune préférence !"
    };
    println!("{}",resultat);
    match isOk {
        true => {
            println!("C'est OK !");
        }
        false => {
            println!("C'est pas OK !");
        }
    }
    match age {
        17 => {
            println!("mineur !");
        }
        18 => {
            println!("majeur !");
        }
        _ => {
            println!("ni 17, ni 18 !");
        }
    }
    // Switch avec une condition à l'intérieur
    match age {
        tmp if tmp > 17 => {
            println!("Majeur !");
        }
        _ => {
            println!("Mineur !");
        }
    }
    match i {
        10...100 => println!("La variable est entre 10 et 100 (inclus)"),
        p => println!("{} n'est pas entre 10 et 100 (inclus)", i)
    };
    match une_variable {
        "jambon" | "poisson" | "oeuf" => println!("Des protéines !"),
        "bonbon" => println!("Des bonbons !"),
        "salade" | "épinards" | "fenouil" => println!("Beurk ! Des légumes !"),
        _ => println!("ça, je sais pas ce que c'est...")
    }
    // Test dans un switch d'une valeur renvoyée par une fonction
    match fais_quelque_chose(20) {
        Some(s) => println!("{}", &s),
        None => {} // rien à afficher donc on ne fait rien
    }
    // Test dans un if let d'une valeur renvoyée par une fonction
    if let Some(s) = fais_quelque_chose(20) {
        println!("{}", &s)
    } 
    else {
        println!("il ne s'est rien passé")
    }
    // Boucle affichant toutes les valeurs du verteur
    loop {
        match v.pop() {
            Some(x) => println!("{}",x),
            None => break,
        }
    }
    v.push(1);
    v.push(2);
    v.push(3);
    while let Some(x) = v.pop() {
        println!("{}",x);
    }

}
fn fais_quelque_chose(i: i32) -> Option<String> {
    if i < 10 {
        Some("variable inférieure à 10".to_owned())
    } else {
        None
    }
}
fn exercice3 () {
    for i in 0..10 {
        println!("i vaut : {}", i);
    }

    let v = vec!(1, 4, 5, 10, 6);
    for value in v {
        println!("{}", value);
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {} et j = {}", i, j);
    }
    let v = vec!("a", "b", "c", "d");

    for (i, value) in v.iter().enumerate() {
        println!("i = {} et value = \"{}\"", i, value);
    }
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // on continue la boucle sur x
            if y % 2 == 0 { continue 'inner; } // on continue la boucle sur y
            println!("x: {}, y: {}", x, y);
        }
    }
    println!("Deuxième boucle !");
    'global: for _ in 0..10 {
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x > 3 { break 'global; } // on arrête la boucle qui s'appelle global
                if x % 2 == 0 { continue 'outer; } // on continue la boucle sur x
                if y % 2 == 0 { continue 'inner; } // on continue la boucle sur y
                println!("x: {}, y: {}", x, y);
            }
        }
    }
}
fn exercice4 () {
    println!("1 + 2 = {}", addition(1, 2));
}
fn addition(nb1: i32, nb2: i32) -> i32 {
    nb1 + nb2
}
fn exercice5 () {
    println!("{}",test_expression(5));
}
// Les expressions renvoie directement un résultat sans return ni même ;
fn test_expression(x: i32) -> i32 {
    if x < 0 {
        println!("{} < 0", x);
        -1
    } else if x == 0 {
        println!("{} == 0", x);
        0
    } else {
        println!("{} > 0", x);
        1
    }
}