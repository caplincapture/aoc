use std::str::FromStr;
use std::time::{Duration, Instant};

// 👋 new!
use itertools::{process_results, Itertools};
use color_eyre::eyre::Result;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<outcome>EOF, got {s:?}"));
        };
        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { theirs, ours })
    }
}

impl Move {
    /// How many points do we get for picking that move?
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Move {
    fn outcom(self, theirs: Move) -> Outcome {
        match (self, theirs) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }
}

impl Move {
    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move")
    }

    fn drawing_move(self) -> Self {
        self
    }
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        }
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcom(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    use std::time::{Duration, Instant};
    let start = Instant::now();

    let total_score: usize = itertools::process_results(
        include_str!("../input.txt")
            .lines()
            .map(Round::from_str)
            //    there 👇   👇
            .map_ok(|r| dbg!(dbg!(r).our_score())),
        |it| it.sum(),
    )?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    dbg!(total_score);

    Ok(())
}