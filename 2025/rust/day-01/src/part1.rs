use miette::{IntoDiagnostic, miette};
use pest::Parser;
use pest_derive::Parser;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let mut dial: i32 = 50;
    let mut number_of_times_landed_at_zero = 0;
    let file = InputParser::parse(Rule::file, input)
        .into_diagnostic()?
        .next()
        .unwrap();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::EOI => (),
            Rule::direction => {
                return Err(miette!(
                    "Should not reach here"
                ));
            }
            Rule::turns => {
                return Err(miette!(
                    "Should not reach here"
                ));
            }
            Rule::instruction => {
                let direction = {
                    let mut inner_rules =
                        record.into_inner();
                    let direction: &str = inner_rules
                        .next()
                        .unwrap()
                        .as_str();
                    let turns: &str = inner_rules
                        .next()
                        .unwrap()
                        .as_str();
                    let turns: u32 =
                        turns.parse().into_diagnostic()?;
                    let direciton = direction.chars().next().ok_or(miette!("Failed to get first char of direction"))?;

                    let direction =
                        Direction::new(direciton, turns)?;
                    Ok::<Direction, miette::Error>(
                        direction,
                    )
                }?;
                match direction {
                    Direction::Left(turns) => {
                        let turns: i32 = turns
                            .try_into()
                            .into_diagnostic(
                        )?;
                        dial =
                            (dial - turns).rem_euclid(100)
                    }
                    Direction::Right(turns) => {
                        let turns: i32 = turns
                            .try_into()
                            .into_diagnostic(
                        )?;
                        dial =
                            (dial + turns).rem_euclid(100)
                    }
                }
                if dial == 0 {
                    number_of_times_landed_at_zero =
                        number_of_times_landed_at_zero + 1;
                }
                println!(
                    "instuction {:?}, dial {}",
                    direction, dial
                );
            }
            Rule::file => {
                return Err(miette!(
                    "Should not reach here"
                ));
            }
        }
    }
    // TODO: Get
    // todo!("day-01 - part 1");
    Ok(number_of_times_landed_at_zero)
}

#[derive(Debug)]
enum Direction {
    Left(u32),
    Right(u32),
}

impl Direction {
    fn new(
        direction: char,
        turns: u32,
    ) -> miette::Result<Self> {
        if direction == 'L' {
            Ok(Self::Left(turns))
        } else if direction == 'R' {
            Ok(Self::Right(turns))
        } else {
            Err(miette!("Not valid turn direction"))
        }
    }
}

#[derive(Parser)]
#[grammar = "part1.pest"]
struct InputParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(3, process(input)?);
        Ok(())
    }
}
