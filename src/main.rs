mod enigma;

fn main() {
    //println!("Hello, world!");
    let list_char = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J','K',
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z'
    ];

    let code_map = enigma::parts::code_map::create(list_char);
    print!("S -> {}\n", code_map.convert_to_code(&'S'));
    print!("{} -> 19\n", code_map.convert_to_char(&19));
}
