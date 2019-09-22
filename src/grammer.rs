use crate::parser::Span;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Reg {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    Constant(i128),
    Variable(&'a str),

    Negative(Box<Expr<'a>>),

    Plus(Box<Expr<'a>>, Box<Expr<'a>>),
    Minus(Box<Expr<'a>>, Box<Expr<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a> {
    Declaration {
        variable: &'a str,
        register: Reg,
    },
    Assignment {
        working_set: Vec<Reg>,
        computation: Expr<'a>,
        output: &'a str,
    },
    If {
        working_set: Vec<Reg>,
        condition: Expr<'a>,
        body: Vec<(Span<'a>, Statement<'a>)>,
    },
    FunctionCall {
        function: &'a str,
        arguments: Vec<&'a str>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<&'a str>,
    pub body: Vec<(Span<'a>, Statement<'a>)>,
    pub span: Span<'a>,
}
