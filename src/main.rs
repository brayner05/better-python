use std::{fs::{self, File}, io::{self, Read, Write}};
use color_eyre::*;

mod lexer;
mod parser;
mod transpiler;

///
/// Continouously reads lines from the user until the specified exit command
/// is entered. Then for every line entered, considers that line to be an expression,
/// and then computes the result_value of that expression.
/// 
fn run_repl() {
    let mut line = String::new();
    'repl: loop {
        print!("expr > ");
        io::stdout().flush().unwrap();

        // Read an expression from the user
        io::stdin()
            .read_line(&mut line)
            .unwrap();

        // If the user entered the quit command, break out of the REPL.
        if line.trim() == String::from(".quit") {
            break 'repl;
        }

        let lexer_result = lexer::scan_all_tokens(line.trim());
        if let Err(e) = &lexer_result {
            eprintln!("{}", e);
        }

        let mut token_stream = lexer_result.unwrap();
        
        let parser_result = parser::generate_ast(&mut token_stream);
        if let Err(e) = &parser_result {
            eprintln!("{}", e);
        }

        let ast = parser_result.unwrap();
        // for stmt in &ast {
        //     println!("{}", stmt.as_ref())
        // }

        for statement in &ast {
            let output = transpiler::transpile(statement.clone());
            if let Err(e) = &output {
                eprintln!("{}", e);
            }
            println!("{}", output.unwrap());
        }

        line.clear();
    }
}


fn run_file(file: &mut File) -> eyre::Result<()> {
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    let mut tokens = lexer::scan_all_tokens(&source)?;
    let ast = parser::generate_ast(&mut tokens)?;
    
    for file_level_stmt in &ast {
        let translation = transpiler::transpile(file_level_stmt.clone())?;
        println!("{}", translation);
    }

    Ok(())
}


fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() < 2 {
        run_repl();
        return Ok(());
    }

    let source_path = arguments[1].clone();
    let mut source_file = fs::File::open(source_path)?;

    let result = run_file(&mut source_file);

    if let Err(e) = &result {
        eprintln!("{}", &result.unwrap_err());
    }
    
    Ok(())
}
