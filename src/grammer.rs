#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    Constant(i128),
    Variable(&'a str),

    Negative(Box<Expr<'a>>),

    Plus(Box<Expr<'a>>, Box<Expr<'a>>),
    Minus(Box<Expr<'a>>, Box<Expr<'a>>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Statement<'a> {
    Declaration {
        variable: &'a str,
        register: &'a str,
    },
    Assigment {
        working_set: Vec<&'a str>,
        computation: Expr<'a>,
        output: &'a str,
    },
    If {
        working_set: Vec<&'a str>,
        condition: Expr<'a>,
        body: Vec<Statement<'a>>,
    },
    FunctionCall {
        function: &'a str,
        arguments: Vec<&'a str>,
    }
}

pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<&'a str>,
    pub body: Vec<Statement<'a>>,
}
