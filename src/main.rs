fn main() {
    let input = "fn hello() {
    res a = true;
}";
    match kon_parser::program(input) {
        Ok(ast) => {
            println!("parsed:\n{:#?}", ast);
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}
