fn main() {
    let input = r#"
fn main() -> str {
    let a = @(1, 2, 3, 4, 5);
}
        "#;

    match kon_parser::program(input) {
        Ok(ast) => {
            println!("parsed:\n{:#?}", ast);
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}
