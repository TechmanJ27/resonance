use nom::{
  IResult,
  Parser,
  sequence::delimited,
  character::complete::char,
  bytes::complete::is_not
};

const RESERVED_KEYWORDS: String[] = {};

#[derive(Debug, PartialEq, Clone)]
enum Statement<'a> {
    Object(Box<IndexMap<&'a str, JsonNode<'a>>>),
    Array(Vec<JsonNode<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";

  take_while(move |c| chars.contains(c))(i)
}

fn boolean_expression<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&str, &str> {
  let expression = tuple(val, opt(sp), boolean_operator, opt(sp), val);
  tuple(opt(char('!')), delimited(char('('), expression, char(')'))).parse(i)
}

fn boolean_operator<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&str, &str> {
  alt(tag("=="), tag("!="), char('<'), char('>'), tag(">="), tag("<=")).parse(i)
}

fn logical_operator<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&str, &str> {
  alt(tag("||"), tag("&&")).parse(i)
}

fn condition<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&str, &str> {
  alt(
    boolean_expression,
    tuple(boolean_expression, opt(sp), logical_operator, opt(sp), boolean_expression)
  )
  .parse(i)
}

fn parse(i: &str) {
    map(
      delimeted(
        condition
      ),
      |v| Statement::Object(Box::new(v.into_iter().collect())),
    )(i)
}
