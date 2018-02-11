#[macro_use]
extern crate reflex;

use std::io;
use std::io::Read;
use std::fmt;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let ruleset = init_ruleset();
    let program = env::args().nth(1).unwrap();
    interpret(reflex::lex(&ruleset, program));
}

fn init_ruleset() -> reflex::Ruleset<Token> {
    let mut ruleset = reflex::Ruleset::<Token>::new();
    ruleset.add_rule(r"-[0-9]*\.?[0-9]+", lex_rule!(|tok| Token::Number(-tok.parse::<f64>().unwrap())));
    ruleset.add_rule(r"[0-9]*\.?[0-9]+", lex_rule!(|tok| Token::Number(tok.parse().unwrap())));
    ruleset.add_rule(r"[^0-9$:\s]+", lex_rule!(|tok| Token::Identifier(tok.to_string())));
    ruleset.add_simple(r"\$", Token::ValueOf);
    ruleset.add_simple(r":", Token::Assign);
    ruleset.add_noop(r"(?s)\s");
    ruleset
}

fn read_stdin() -> String {
    let mut program = String::new();
    io::stdin().read_to_string(&mut program)
        .expect("Unable to read stdin");
    program
}

macro_rules! error {
    ($($format_arg:expr),*) => { {
        println!($($format_arg,)*);
        std::process::exit(1);
    } }
}

fn interpret(tokens_i: reflex::Lexer<Token>) {
    let mut tokens: VecDeque<Token> = tokens_i.map(|res| res.unwrap()).collect();
    let mut variables: HashMap<String, f64> = HashMap::new();
    variables.insert("E".to_string(), std::f64::consts::E);
    variables.insert("PI".to_string(), std::f64::consts::PI);
    let mut functions: HashMap<String, Box<Fn(&mut Vec<f64>)>> = HashMap::new();
    define_builtins(&mut functions);
    let mut stack: Vec<f64> = Vec::new();
    let args: Vec<String> = read_stdin().split_whitespace().map(|s| s.to_string()).collect();
    while !tokens.is_empty() {
        let token: Token = tokens.pop_front().unwrap();
        match token {
            Token::ValueOf => {
                //println!("Processing ValueOf");
                stack.push(match tokens.pop_front().unwrap() {
                    Token::Number(n)         => args[(n as usize)+1].parse().unwrap(),
                    Token::Identifier(ref s) => *variables.get(s).unwrap(),
                    _                        => error!("Syntax error: $ not followed by number or identifier")
                });
            },
            Token::Assign => {
                //println!("Processing Assign");
                if let Token::Identifier(ref s) = tokens.pop_front().unwrap() {
                    variables.insert(s.clone(), stack.pop().unwrap());
                } else { error!("Syntax error: : not followed by identifier"); }
            },
            Token::Number(n) => {
                //println!("Processing number: {}", n);
                stack.push(n);
            },
            Token::Identifier(ref s) => {
                match functions.get(s) {
                    Some(func) => func(&mut stack),
                    None       => error!("Unknown function \"{}\"", s),
                };
                //functions.get(s).unwrap()(&mut stack);
            },
        };
    };
}

/* Called like so:
   To push a value onto the stack:
    builtin!(functions, "name"; arg2, arg1 -> some_func(arg1, arg2));
   To push nothing and do some side effect (e.g. printing):
    builtin!(functions, "name"; arg2, arg1 -> {do_a_thing(arg1, arg2);});
   Unfortunately you have to give the arguments in reverse order as a result of the combination of
   expansion and pop semantics.
*/
macro_rules! builtin {
    ($hash:ident, $name:expr; $($argument:ident),* -> $body:block) => {
        $hash.insert($name.to_string(), Box::new(|stack: &mut Vec<f64>| {
            $(let $argument = stack.pop().unwrap();)*
            $body;
        }));
    };
    ($hash:ident, $name:expr; $($argument:ident),* -> $body:expr) => {
        $hash.insert($name.to_string(), Box::new(|stack: &mut Vec<f64>| {
            $(let $argument = stack.pop().unwrap();)*
            stack.push($body);
        }));
    };
}

fn define_builtins(functions: &mut HashMap<String, Box<Fn(&mut Vec<f64>)>>) {
    builtin!(functions, "+";        b, a -> a + b);
    builtin!(functions, "-";        b, a -> a - b);
    builtin!(functions, "*";        b, a -> a * b);
    builtin!(functions, "/";        d, n -> n / d);
    builtin!(functions, "^";        b, a -> a.powf(b));
    builtin!(functions, "sqrt";     x -> x.sqrt());
    builtin!(functions, "exp";      x -> x.exp());
    builtin!(functions, "=";        b, a -> (a == b) as u8 as f64);
    builtin!(functions, "<";        b, a -> (a < b) as u8 as f64);
    builtin!(functions, ">";        b, a -> (a > b) as u8 as f64);
    builtin!(functions, "<=";       b, a -> (a <= b) as u8 as f64);
    builtin!(functions, ">=";       b, a -> (a >= b) as u8 as f64);
    builtin!(functions, "?";        iffalse, iftrue, cond ->
                                        if cond != 0.0 { iftrue } else { iffalse });
    builtin!(functions, "."; x -> { println!("{}", x); });
}

#[derive(Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    ValueOf,
    Assign,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Token::Number(num)        => format!("Number: {}", num.to_string()),
            Token::Identifier(ref id) => format!("Identifier: {}", id),
            Token::ValueOf            => "$".to_string(),
            Token::Assign             => ":".to_string(),
        })
    }
}
