use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{one_of, space1},
    combinator::{map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use miette::GraphicalReportHandler;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};

mod parse;
use parse::{parse_all_monkeys, Span, Term, Operation, Monkey};

pub fn parse_term(i: &str) -> IResult<&str, Term> {
    alt((value(Term::Old, tag("old")), map(cc::u64, Term::Constant)))(i)
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

pub fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (l, op, r)) =
        preceded(tag("new = "), tuple((parse_term, one_of("*+"), parse_term)))(i)?;
    let op = match op {
        '*' => Operation::Mul(l, r),
        '+' => Operation::Add(l, r),
        _ => unreachable!(),
    };
    Ok((i, op))
}

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    // Sample input:
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3

    let (i, _) = tuple((space1, tag("Monkey "), cc::u64, tag(":\n")))(i)?;
    // we could use "preceded" but at this point the nesting gets a bit confusing
    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), cc::u64),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;
    let (i, (_, _, divisor, _)) =
        tuple((space1, tag("Test: divisible by "), cc::u64, tag("\n")))(i)?;
    let (i, (_, _, receiver_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, receiver_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;

    Ok((
        i,
        Monkey {
            items_inspected: 0,
            items,
            operation,
            divisor,
            receiver_if_true,
            receiver_if_false,
        },
    ))
}

fn main() {
    let input_static = include_str!("input.txt");
    let input = Span::new(input_static);

    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(parse_all_monkeys::<ErrorTree<Span>>)(input);
    let monkeys = match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { .. } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            }
            return;
        }
    };

    for monkey in &monkeys {
        println!("{monkey:?}");
    }
}