use miette::IntoDiagnostic;
use pest::Parser;
use pest_derive::Parser;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let file = InputParser::parse(Rule::file, input)
        .into_diagnostic()?
        .next()
        .unwrap();
    let mut invalid_ids: Vec<u64> = Vec::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::EOI => (),
            Rule::instruction => {
                let mut inner_rules = record.into_inner();
                let start_number: &str =
                    inner_rules.next().unwrap().as_str();
                let start: u64 = start_number
                    .parse()
                    .into_diagnostic()?;
                // dbg!(start);
                let end_number: &str =
                    inner_rules.next().unwrap().as_str();
                let end: u64 =
                    end_number.parse().into_diagnostic()?;
                let mut new_invalid_ids =
                    process_one(start, end)?;
                invalid_ids.append(&mut new_invalid_ids);
                // dbg!(end);
            }
            Rule::number => panic!(),
            Rule::file => panic!(),
        }
    }
    invalid_ids.dedup();
    let invalid_ids_total: u64 = invalid_ids.iter().sum();
    Ok(format!("{invalid_ids_total}"))
}

#[derive(Parser)]
#[grammar = "parse.pest"]
struct InputParser;

pub fn process_one(
    start_of_range: u64,
    end_of_range: u64,
) -> miette::Result<Vec<u64>> {
    let mut invalid_ids = Vec::new();
    for id in start_of_range..=end_of_range {
        let id_string = format!("{id}");
        // Recursivly split the string and see if it is repeated numbers. 112112 -> 112 == 112, therefore invalid
        let id_string_chars =
            id_string.chars().collect::<Vec<char>>();
        // If it is even
        for split_amount in
            (1..=id_string.len()).filter(|&i| {
                id_string.len() % i == 0
                    // Stops it just matching the whole id to itself
                        && i != id_string.len()
            })
        {
            let chunks: Vec<&[char]> = id_string_chars
                .chunks(split_amount)
                .collect();
            let first_chunk = *chunks
                .get(0)
                .expect("Should have at lest one chunk");
            // dbg!(split_amount, id, first_chunk);
            if chunks.iter().all(|a| *a == first_chunk) {
                invalid_ids.push(id);
                // Don't end up with duplicates
                // break;
            }
            // println!("split amount: {split_amount}")
        }
    }
    invalid_ids.dedup();
    return Ok(invalid_ids);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(11, 22, vec![11,22])]
    #[case(95, 115, vec![99,111])]
    #[case(998, 1012, vec![999,1010])]
    #[case(1188511880, 1188511890, vec![1188511885])]
    #[case(222220, 222224, vec![222222])]
    #[case(1698522, 1698528, vec![])]
    #[case(446443, 446449, vec![446446])]
    #[case(38593856, 38593862, vec![38593859])]
    #[case(565653, 565659, vec![565656])]
    #[case(824824821, 824824827, vec![824824824])]
    #[case(2121212118, 2121212124, vec![2121212121])]
    fn test_process(
        #[case] start_of_range: u64,
        #[case] end_of_range: u64,
        #[case] expected: Vec<u64>,
    ) -> miette::Result<()> {
        assert_eq!(
            expected,
            process_one(start_of_range, end_of_range)?
        );
        Ok(())
    }
}
