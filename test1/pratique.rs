
fn main() {
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

    while true {

    }
}