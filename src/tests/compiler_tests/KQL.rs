use crate::compilers::KQL::lexer::Lexer;

#[test]
fn build_token_tree() {
    let mut lexer = Lexer::from_text(r#"GET my_database.A
FIELDS "name", "email", "pass""#);

    let compile = lexer.parse();

    if let Ok(query) = compile {
        dbg!(query);
        assert_eq!(1, 1)
    } else {
        println!("{:#?}", compile.unwrap_err());
        assert_eq!(1, 1)
    }
}

#[test]
fn test_invalid_token() {
    let mut lexer = Lexer::from_text(r#"my_database.A
FIELDS "name", "email", "pass""#);

    let compile = lexer.parse();

    if let Ok(query) = compile {
        dbg!(query);
        assert_eq!(1, 2)
    } else {
        let errors = compile.unwrap_err();
        println!("{:#?}", errors);
        for error in errors {
            println!("{}", error.formatted_color);
        }
        assert_eq!(1, 1)
    }
}