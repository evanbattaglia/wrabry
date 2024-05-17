use winnow::ascii::multispace1;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

#[derive(Debug)]
struct IdAndAliases<'a> {
    id: &'a str,
    aliases: Vec<&'a str>,
}

fn parse_identifier<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., (AsChar::is_alphanum, '-', '_')).parse_next(input)
}

fn parse_identifier_and_aliases<'s>(input: &mut &'s str) -> PResult<IdAndAliases<'s>> {
    let id = parse_identifier(input)?;
    // TODO aliases
    let res = IdAndAliases {
        id,
        aliases: vec![]
    };
    Ok(res)
}

// for strings, we'll need a String becaues we'll nede to replace the escape sequences. can
// probably use a Cow<str> to avoid allocations and copies

fn parse_sub<'s>(input: &mut &'s str) -> PResult<IdAndAliases<'s>> {
    "sub".parse_next(input)?;
    multispace1(input)?;
    let id_and_aliases = parse_identifier_and_aliases(input)?;

    Ok(id_and_aliases)
}

fn main() {
    let mut test_str = "sub foo-123_cool";
    let id = parse_sub(&mut test_str);
    println!("{:?}", id);
}
