use core::fmt;
use std::collections::BTreeMap;
use std::{collections::HashMap, cell::RefCell};

use std::rc::Rc;

use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Tree, Node};
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}
#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

/* fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
} */


#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

impl FsEntry {
    fn build(mut self, it: &mut dyn Iterator<Item = Line>) -> Self {
        while let Some(line) = it.next() {
            match line {
                Line::Command(Command::Cd(sub)) => match sub.as_str() {
                    "/" => {
                        // muffin,
                    }
                    ".." => break,
                    _ => self.children.push(
                        FsEntry {
                            path: sub.clone(),
                            size: 0,
                            children: vec![],
                        }
                        .build(it),
                    ),
                },
                Line::Entry(Entry::File(size, path)) => {
                    self.children.push(FsEntry {
                        path,
                        size,
                        children: vec![],
                    });
                }
                _ => {
                    // ignore other commands
                }
            }
        }
        self
    }
}

fn main() {
    let mut lines = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }
    .build(&mut lines);
    dbg!(&root);

    // solving part 1 because it's the same difficulty as part 2, just less code
    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s < 100_000)
        .sum::<u64>();
    dbg!(sum);
}
