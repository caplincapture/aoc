use std::collections::{HashSet, HashMap};


#[cfg(test)]
mod tests {

    #[test]
    fn all_good() {
        macro_rules! check {
            ($name:ident) => {
                for input in [include_str!("../input.txt"), include_str!("../input.txt")] {
                    assert_eq!(crate::naive(input), crate::$name(input));
                }
            };
        }

        check!(state_structs);
        check!(arrays);
        check!(small_vecs);
    }
}

// I couldn't resist doing this: instead of having the input be a byte slice
// (&[u8]) and test against b'<' and b'>', I did do a tiny bit of parsing first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Dir::Left),
            '>' => Ok(Dir::Right),
            // minimal error handling here, we return the character if it wasn't
            // one of '<' or '>'
            _ => Err(c),
        }
    }
}

pub fn naive(i: &str) -> usize {
    // same as before
    
    let jets = i
        .chars()
        .map(|c| Dir::try_from(c).unwrap())
        .collect::<Vec<_>>();

    let rocks = vec![
        vec![[2, 0], [3, 0], [4, 0], [5, 0]],
        vec![[2, 1], [3, 1], [3, 2], [3, 0], [4, 1]],
        vec![[2, 0], [3, 0], [4, 0], [4, 1], [4, 2]],
        vec![[2, 0], [2, 1], [2, 2], [2, 3]],
        vec![[2, 0], [3, 0], [2, 1], [3, 1]],
    ];

    // we've got a few more variables here
    let mut chamber = (0..7).map(|x| (x, 0)).collect::<HashSet<_>>();
    let mut highest = 0;
    let mut jet: usize = 0;
    // XXX: 💀
    // the above is my original comment on this: this is extremely loosely
    // typed.  the key are 7 y-coordinates, which correspond to the height of
    // the rocks in the chamber, relative to the lowest one of them - see below
    // for a diagram.
    let mut states = HashMap::<Vec<usize>, Vec<usize>>::new();
    let mut total_rocks = 0;
    let mut rock_index: usize = 0;
    let mut cycle_found = false;
    let mut height_gain_in_cycle = 0;
    let mut skipped_cycles = 0;

    // this is now a while loop, since we need to skip ahead using the cycles.
    while total_rocks < 1000000000000 {
        let mut rock = rocks[rock_index].clone();
        let adjustment = highest + 4;
        // set the rock's position to be at the current drop height
        for n in &mut rock {
            n[1] += adjustment;
        }
        let mut rest = false;
        while !rest {
            // move the rock left or right, as needed
            let mut new_rock = vec![];
            if jets[jet] == Dir::Left {
                // if we're moving left, update the rock to be one to the left
                if rock[0][0] > 0 {
                    let mut good = true;
                    for n in &rock {
                        if chamber.contains(&(n[0] - 1, n[1])) {
                            good = false;
                            break;
                        }
                        new_rock.push([n[0] - 1, n[1]]);
                    }
                    if good {
                        rock = new_rock;
                    }
                }
            } else {
                // if we're moving right, update the rock to be one to the right
                if rock.last().unwrap()[0] < 6 {
                    let mut good = true;
                    for n in &rock {
                        if chamber.contains(&(n[0] + 1, n[1])) {
                            good = false;
                            break;
                        }
                        new_rock.push([n[0] + 1, n[1]]);
                    }
                    if good {
                        rock = new_rock;
                    }
                }
            }

            jet = (jet + 1) % jets.len();

            // move down if we can
            {
                new_rock = vec![];
                let mut good = true;
                for n in &rock {
                    if chamber.contains(&(n[0], n[1] - 1)) {
                        for m in &rock {
                            chamber.insert((m[0], m[1]));
                        }
                        rest = true;
                        highest = rock
                            .iter()
                            .map(|n| n[1])
                            .chain([highest].into_iter())
                            .max()
                            .unwrap();
                        good = false;
                        break;
                    } else {
                        new_rock.push([n[0], n[1] - 1]);
                    }
                }
                if good {
                    rock = new_rock;
                }
            }
        } // while !rest

        total_rocks += 1;

        // here's where cycle detection happens. we only try it if we haven't
        // already found a cycle earlier (which means we already skipped ahead,
        // which means we can just jump forward)
        if !cycle_found {
            // let's say the chamber looks like this:
            // 
            //   #     --- y = 5
            //  ###  # --- y = 4
            //  ### ## --- y = 3
            // ####### --- y = 2
            // ####### --- y = 1
            // ####### --- y = 0
            //
            // then after this loop, we'd have:
            // state = vec![2, 4, 5, 4, 2, 3, 4]

            let mut state: Vec<usize> = vec![];
            for i in 0..7 {
                state.push(
                    chamber
                        .iter()
                        .filter(|x| x.0 == i)
                        .map(|x| x.1)
                        .max()
                        .unwrap(),
                )
            }
            let lowest = state.iter().copied().min().unwrap();
            // lowest is `2`

            let mut state = state.into_iter().map(|x| x - lowest).collect::<Vec<_>>();
            // and now we have:
            // state = vec![0, 2, 3, 2, 0, 1, 2]

            // and the last two numbers (our "state key") are our positions into
            // the rock array and the jet array.
            state.extend([rock_index, jet].into_iter());
            if let Some(state_data) = states.get(&state) {
                // if we've already seen this state, we have a loop!
                height_gain_in_cycle = highest - state_data[0];
                let rocks_in_cycle = total_rocks - state_data[1];
                // use it to skip ahead.
                skipped_cycles = (1000000000000 - total_rocks) / rocks_in_cycle;
                total_rocks += skipped_cycles * rocks_in_cycle;
                cycle_found = true;
            } else {
                states.insert(state, vec![highest, total_rocks]);
            }
        }

        rock_index = (rock_index + 1) % 5;
    }

    highest + (skipped_cycles * height_gain_in_cycle)
}
