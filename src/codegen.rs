use crate::grammer::*;
use std::collections::HashMap;
use std::fmt::Write;

use crate::parser::Span;

static ARGUMENT_REGISTERS: [Reg; 6] = [Reg::Rdi, Reg::Rsi, Reg::Rdx, Reg::Rcx, Reg::R8, Reg::R9];

#[derive(Clone, PartialEq)]
struct Scope<'a> {
    start: Span<'a>,
    end: Option<Span<'a>>,
}

struct Codegen<'a> {
    errors: String,

    // Output code.
    output: String,

    // Map from variable name to the values it has in each scope.
    variables: HashMap<&'a str, Vec<(Scope<'a>, Reg)>>,
}

impl<'a> Codegen<'a> {
    fn append_error(&mut self, span: Span, error: &str) {
        writeln!(&mut self.errors, "{}:{} {}", span.extra, span.line, error).unwrap();
    }

    fn resolve(&self, var: &str) -> Option<Reg> {
        self.variables
            .get(var)
            .and_then(|v| v.last())
            .filter(|s| s.0.end.is_none())
            .map(|s| s.1)
    }

    fn compile_expr(&mut self, span: Span, expr: &Expr, working_set: &[Reg], output: Reg) {
        match expr {
            // Expr::Constant(c) => writeln!(self.output, " movq ${}, {}", c, output),
            // Expr::Variable(v) => {if let Some(r) = self.resolve(v) {
            //     writeln!(self.output, " movq {}, {}", self.register(v), output);
            // } else {
            //     self.append_error(span, &format!("variable '{}' not in scope", v));
            // }
            // }
            // Expr::Plus(box Expr::Variable(v), expr)
            _ => {}
        }
    }

    fn compile_statement(&mut self, span: Span, statement: &Statement) {
        match statement {
            Statement::Assignment {
                computation,
                output,
                working_set,
            } => {
                self.compile_expr(
                    span,
                    computation,
                    &working_set[..],
                    self.resolve(output).unwrap(),
                );
            }
            _ => unimplemented!(),
        }
    }
}

pub fn run(func: &Function) -> Result<String, String> {
    let mut codegen = Codegen {
        errors: String::new(),
        output: String::new(),
        variables: func
            .arguments
            .iter()
            .enumerate()
            .map(|(i, &n)| {
                (
                    n,
                    vec![(
                        Scope {
                            start: func.span,
                            end: None,
                        },
                        ARGUMENT_REGISTERS[i],
                    )],
                )
            })
            .collect(),
    };

    writeln!(&mut codegen.output, ".globl {}", func.name).unwrap();
    writeln!(&mut codegen.output, "{}:", func.name).unwrap();

    for (span, statement) in &func.body {
        codegen.compile_statement(*span, &statement);
    }

    if !codegen.errors.is_empty() {
        return Err(codegen.errors);
    }
    Ok(codegen.output)
}
