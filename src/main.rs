use ariadne::{Label, Report, ReportKind, Source};

fn line_col_to_offset(source: &str, line: usize, col: usize) -> usize {
    source
        .lines()
        .take(line - 1)
        .map(|l| l.len() + 1) // +1 for newline
        .sum::<usize>()
        + col
        - 1
}

fn main() {
    let input = "fn hello() {

;";
    match kon_parser::program(input) {
        Ok(ast) => {
            println!("parsed:\n{:?}", ast);
        }
        Err(err) => {
            let start = line_col_to_offset(input, err.location.line, err.location.column);
            let span = ("<input>", start..=start);

            Report::build(ReportKind::Error, span.clone())
                .with_message("Parse error")
                .with_label(Label::new(span).with_message(format!("expected {:?}", err.expected)))
                .finish()
                .print(("<input>", Source::from(input)))
                .unwrap();
        }
    }
}
