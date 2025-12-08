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
                let (
                    dial_new,
                    number_of_times_landed_at_zero_instruction,
                ) = process_turn(dial, &direction)?;
                dial = dial_new;
                number_of_times_landed_at_zero += number_of_times_landed_at_zero_instruction;
            }
            Rule::file => {
                return Err(miette!(
                    "Should not reach here"
                ));
            }
        }
    }
    Ok(number_of_times_landed_at_zero)
}

const DIAL_MAX: i32 = 100;
fn process_turn(
    start_dial: i32,
    instruction: &Direction,
) -> miette::Result<(i32, i32)> {
    match instruction {
        Direction::Left(turns) => {
            let turns: i32 = turns
                .clone()
                .try_into()
                .into_diagnostic()?;
            let end_dial = start_dial - turns;
            let password: i32 = get_password_from_turn(
                start_dial, end_dial,
            );
            Ok((end_dial.rem_euclid(DIAL_MAX), password))
        }
        Direction::Right(turns) => {
            let turns: i32 = turns
                .clone()
                .try_into()
                .into_diagnostic()?;
            let end_dial = start_dial + turns;
            let password: i32 = get_password_from_turn(
                start_dial, end_dial,
            );
            Ok((end_dial.rem_euclid(DIAL_MAX), password))
        }
    }
}

fn get_password_from_turn(
    start_dial: i32,
    end_dial: i32,
) -> i32 {
    let mut working_password = 0;
    if (start_dial > 0 && end_dial <= 0)
        || (start_dial < 0 && start_dial >= 0)
    {
        working_password += 1;
    }
    if end_dial.abs() >= DIAL_MAX {
        working_password += end_dial.abs() / DIAL_MAX;
    }
    working_password
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
    use rstest::rstest;

    #[rstest]
    #[case(50, Direction::new('L', 68).unwrap(), 82, 1)]
    #[case(82, Direction::new('L', 30).unwrap(), 52, 0)]
    #[case(52, Direction::new('R', 48).unwrap(), 0, 1)]
    #[case(0, Direction::new('L', 5).unwrap(), 95, 0)]
    #[case(95, Direction::new('R', 60).unwrap(), 55, 1)]
    #[case(55, Direction::new('L', 55).unwrap(), 0, 1)]
    #[case(0, Direction::new('L', 1).unwrap(), 99, 0)]
    #[case(99, Direction::new('L', 99).unwrap(), 0, 1)]
    #[case(0, Direction::new('R', 14).unwrap(), 14, 0)]
    #[case(14, Direction::new('L', 82).unwrap(), 32, 1)]
    #[case(10, Direction::new('L', 3000).unwrap(), 10, 30)]
    #[case(10, Direction::new('R', 3000).unwrap(), 10, 30)]
    #[case(1, Direction::new('L', 2).unwrap(), 99, 1)]
    #[case(99, Direction::new('R', 2).unwrap(), 1, 1)]
    #[case(99, Direction::new('R', 1).unwrap(), 0, 1)]
    #[case(0, Direction::new('R', 1).unwrap(), 1, 0)]
    #[case(0, Direction::new('L', 1).unwrap(), 99, 0)]
    #[case(50, Direction::new('R', 1000).unwrap(), 50, 10)]
    fn test_process(
        #[case] start_dial: i32,
        #[case] input: Direction,
        #[case] end_dial: i32,
        #[case] number_of_times_passed_zero: i32,
    ) -> miette::Result<()> {
        assert_eq!(
            (end_dial, number_of_times_passed_zero),
            process_turn(start_dial, &input)?
        );
        Ok(())
    }
}
