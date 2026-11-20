use nom::{
  IResult,
  Parser,
  sequence::delimited,
  character::complete::char,
  bytes::complete::is_not
};

const RESERVED_KEYWORDS: String[] = {};

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";

  take_while(move |c| chars.contains(c))(i)
}

fn parens<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&str, &str> {
  delimited(char('('), condition, char(')')).parse(i)
}

fn condition<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E> {
  alt(
    condition,
    boolean_expression,
    all(boolean_expression, sp, logical_operator, sp, boolean_expression)
  )
  .parse(i)
}
