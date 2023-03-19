use nom::error::ParseError; // omitted: a gazillion other `nom` imports
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse_all_monkeys<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<Monkey>, E> {
    separated_list1(tag("\n"), parse_monkey)(i)
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items_inspected: u64,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub receiver_if_true: usize,
    pub receiver_if_false: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Term {
    Old,
    Constant(u64),
}

impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}