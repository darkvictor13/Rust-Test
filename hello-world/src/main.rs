fn main() {
    let mut range: u8 = 5;

    if range == 5 {
        println!("Entrei no if\n");
    }else {
        println!("Entrei no else\n");
    }
    range = 6;
    
    for number in 0..range {
        print!("Iteração {} ", number);
        println!("Hello, world!");
    }
}
