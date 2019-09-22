
use std::collections::HashMap;
use crate::grammer::*

static ARGUMENT_REGISTERS: [&'static str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"]

struct Variable<'a> {
    name: &'a str,

    scopes: 
}

struct Codegen<'a> {
    errors: String,

    // Output code.
    output: String,

    // Mapping from variable to register used to store that register.
    variables: HashMap<&'a str, &'a str>,

    // Line that each variable is clobbered on.
    clobber_line: HashMap<&'a str>,
}

impl Codegen {
    fn compile_statement(&mut self, line: usize, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Assignment { }

        }
    }
}

pub fn run(func: Function) -> Result<String, String> {
    let mut codegen = Codegen {
        errors: String::new(),
        output: String::new(),
        variables: func.arguments.enumerate().map(|(i, n)| (n, ARGUMENT_REGISTERS[i])),
        colobber_line: HashMap::new(),
    }

    writeln!(&mut codegen.output, ".globl {}", func.name);
    writeln!(&mut codegen.output, "{}:", func.name);

    for &(line, statement) in &func.body {
        codegen.compile_statement(line, &mut statement)?;
    }
}
