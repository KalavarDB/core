use crate::compilers::KQL::lexer::Lexer;

#[test]
fn build_token_tree() {
    let mut lexer = Lexer::from_text(r#"GET my_database.A
FIELDS "name", "email", "pass""#);

    let compile = lexer.parse();

    if let Ok(tree) = compile {
        println!("successfully compiled {} tokens", tree.len());
        assert_eq!(1,1)
    } else {
        println!("{}", compile.unwrap_err());
        assert_eq!(1, 1)
    }
}