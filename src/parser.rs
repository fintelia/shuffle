use nom::{
    branch::alt, bytes::complete::tag, character::complete::*, combinator::*, multi::*,
    sequence::*, IResult,
};
use std::str::FromStr;

use nom_locate::{position, LocatedSpanEx};

use crate::grammer::*;

type Span<'a> = LocatedSpanEx<&'a str, &'a str>;

fn variable(i: Span) -> IResult<Span, &str> {
    map(
        delimited(
            multispace0,
            preceded(peek(alpha1), alphanumeric0),
            multispace0,
        ),
        |s: Span| s.fragment,
    )(i)
}

fn variable_list(i: Span) -> IResult<Span, Vec<&str>> {
    separated_list(tag(","), variable)(i)
}

fn working_set(i: Span) -> IResult<Span, Vec<&str>> {
    map(opt(delimited(tag("["), variable_list, tag("]"))), |o| {
        o.unwrap_or(Vec::new())
    })(i)
}

fn expr(i: Span) -> IResult<Span, Expr> {
    let (i, _) = multispace0(i)?;

    let (i, left) = alt((
        delimited(tag("("), expr, tag(")")),
        map(variable, Expr::Variable),
        map(preceded(tag("0x"), hex_digit1), |c: Span| {
            Expr::Constant(i128::from_str_radix(c.fragment, 16).unwrap())
        }),
        map(digit1, |c: Span| {
            Expr::Constant(i128::from_str(c.fragment).unwrap())
        }),
        map(preceded(tag("-"), expr), |e| Expr::Negative(Box::new(e))),
    ))(i)?;

    Ok(match tuple((one_of("+-"), expr))(i) {
        Ok((i, ('+', right))) => (i, Expr::Plus(Box::new(left), Box::new(right))),
        Ok((i, ('-', right))) => (i, Expr::Minus(Box::new(left), Box::new(right))),
        _ => (i, left),
    })
}

fn assignment(i: Span) -> IResult<Span, Statement> {
    map(
        tuple((working_set, variable, tag("="), expr, tag(";"))),
        |(working_set, output, _, computation, _)| Statement::Assigment {
            working_set,
            output,
            computation,
        },
    )(i)
}

fn if_statement(i: Span) -> IResult<Span, Statement> {
    map(
        tuple((
            working_set,
            multispace0,
            tag("if"),
            multispace1,
            expr,
            delimited(tag("{"), many0(statement), tag("}")),
        )),
        |(working_set, _, _, _, condition, body)| Statement::If {
            working_set,
            condition,
            body,
        },
    )(i)
}

fn function_call(i: Span) -> IResult<Span, Statement> {
    map(
        tuple((
            multispace0,
            alphanumeric0,
            multispace0,
            delimited(tag("("), variable_list, tag(")")),
            multispace0,
            tag(";"),
        )),
        |(_, function, _, arguments, _, _)| Statement::FunctionCall {
            function: function.fragment,
            arguments,
        },
    )(i)
}

fn statement(i: Span) -> IResult<Span, Statement> {
    alt((assignment, if_statement, function_call))(i)
}

fn function(i: Span) -> IResult<Span, Function> {
    map(
        tuple((
            multispace0,
            tag("fn"),
            multispace1,
            alphanumeric1,
            multispace0,
            delimited(tag("("), variable_list, tag(")")),
            multispace0,
            delimited(tag("{"), many0(statement), tag("}")),
        )),
        |(_, _, _, name, _, arguments, _, body)| Function {
            name: name.fragment,
            arguments,
            body,
        },
    )(i)
}

pub fn parse<'a>(filename: &'a str, contents: &'a str) -> IResult<Span<'a>, Vec<Function<'a>>> {
    all_consuming(many0(function))(Span::new_extra(contents, filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Box<Expr> {
        Box::new(Expr::Variable(s))
    }

    fn s(s: &str) -> Span {
        Span::new_extra(s, "filename.hs")
    }

    #[test]
    fn test_variable() {
        assert_eq!(all_consuming(variable)(s("abc")).unwrap().1, "abc");
        assert_eq!(all_consuming(variable)(s("a0c")).unwrap().1, "a0c");
        assert_eq!(all_consuming(variable)(s(" b")).unwrap().1, "b");
        assert_eq!(all_consuming(variable)(s("abc\t")).unwrap().1, "abc");
        assert_eq!(all_consuming(variable)(s("\t \tabc\t")).unwrap().1, "abc");
        assert!(all_consuming(variable)(s("0abc")).is_err());
    }

    #[test]
    fn test_expr() {
        assert_eq!(
            all_consuming(expr)(s("ab+c")).unwrap().1,
            Expr::Plus(v("ab"), v("c"))
        );
        assert_eq!(
            all_consuming(expr)(s("a+(b-c)")).unwrap().1,
            Expr::Plus(v("a"), Box::new(Expr::Minus(v("b"), v("c"))))
        );
    }
}
