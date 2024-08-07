pub fn string_ownership() {
    println!("Hello, world!");
    let string_1 = String::from("Hello Rihana"); //string_1 owns Rihana
    let string_3 = takes_ownership(string_1); //string_3 own Rihana
    println!("{}", string_3);
}
pub fn takes_ownership(string_2: String)->String { //string_2 owns Rihana
    println!("{}", string_2);
    return string_2; //returning ownership to string_3
}
