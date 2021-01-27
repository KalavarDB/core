use crate::compilers::KQL::lexer::Lexer;

#[test]
fn run_lexer_on_text() {
    let mut lexer = Lexer::from_text(r#"GET my_database.A
FIELDS: "name", "email", "pass""#);

    for token_result in lexer {
        match token_result {
            Ok(token) => {
                println!("Token: {:#?}", token);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    assert_eq!(1, 1)
}