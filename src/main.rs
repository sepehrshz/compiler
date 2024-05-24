use crate::lexial::Lexer;
pub(crate) mod lexial;
pub(crate) mod syntax;
pub(crate) mod token;

fn main() {
    // let input = r#"
    //     int x = 2;
    //     // sajkd hasklf j
    // void DAQ_signal_handler_IO ( int status )
    // {
    //     wait_flag = FALSE;
    // }

    // int main() {
    // print("hello word");
    // }
    //     "#
    // .to_string();
    let input = r#"
    int main(){
        int x = 2;
        //sajk hasklf j
        int DAQ_signal_handler_IO ( int status ) {
            wait_flag = FALSE;
        }
    }"#
    .to_string();
    let mut lexer = Lexer::new(input, false);

    let mut tokens = Vec::new();
    while !lexer.is_end() {
        let token = lexer.next_token();
        println!("{:?}", token);
        tokens.push(token);
    }

    match syntax::parse_program(&tokens) {
        Ok(ast) => {
            println!("{:?}", ast);
        }
        Err(err) => {
            println!("Syntax error: {}", err);
        }
    }
}
