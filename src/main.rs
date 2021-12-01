use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input.txt");

type Parts<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn parse_input<'a>() -> (usize, Parts<'a>) {
    let mut parts = HashMap::new();
    let mut lines = INPUT.lines();

    // First line contains the number of missing parts
    // in the following format: "{} onderdelen missen"
    let line = lines.next().unwrap();
    let missing_parts = line.split_whitespace().nth(0).unwrap().parse().unwrap();

    // Each following line is in the format "part: requirements"
    // Requirements are separated by commas in the format "amount part"
    for line in lines {
        let mut split = line.split(": ");
        let part = split.next().unwrap();
        let requirements = split.next().unwrap();

        let requirements_split = requirements.split(", ");
        let mut requirements_parts = Vec::new();

        for requirement in requirements_split {
            let mut requirement_split = requirement.split(" ");
            let requirement_amount = requirement_split.next().unwrap().parse().unwrap();
            let requirement_part = requirement_split.next().unwrap();

            requirements_parts.push((requirement_part, requirement_amount));
        }

        parts.insert(part, requirements_parts);
    }

    (missing_parts, parts)
}

fn get_number_of_parts(part: &str, parts: &Parts) -> usize {
    // Return the sum of the number of parts that can be made from the given part recursively
    // Return 1 if part does not exist
    parts.get(part).map_or(1, |v| {
        v.iter()
            .map(|&(p, n)| n * get_number_of_parts(p, parts))
            .sum()
    })
}

fn get_applicable_parts<'a>(parts: &'a Parts) -> Vec<(&'a str, usize)> {
    // Returns the parts which are not a requirement of any other part
    parts
        .iter()
        .filter(|&(&part, _)| parts.values().all(|v| !v.iter().any(|&(p, _)| p == part)))
        .map(|(&part, _)| (part, get_number_of_parts(part, parts)))
        .collect()
}

fn find_combination_of_missing_parts<'a>(
    number_of_parts: usize,
    number_of_missing_parts: usize,
    applicable_parts: &Vec<(&'a str, usize)>,
) -> Option<HashMap<&'a str, usize>> {
    // TODO: could use some memoization

    // Try to find a combination of parts that can be used to make the number of parts
    if number_of_parts == 0 {
        // If all parts have been used, return the combination
        return if number_of_missing_parts == 0 {
            Some(HashMap::new())
        } else {
            None
        };
    }

    for &(part, amount_of_parts) in applicable_parts {
        let cloned_applicable_parts = applicable_parts
            .iter()
            .filter(|&(p, _)| *p != part)
            .map(|&(p, n)| (p, n))
            .collect();

        for number_of_parts_in_combination in (0..=number_of_parts).rev() {
            let removed_parts = number_of_parts_in_combination * amount_of_parts;
            if removed_parts > number_of_missing_parts {
                break;
            }

            let result = find_combination_of_missing_parts(
                number_of_parts - number_of_parts_in_combination,
                number_of_missing_parts - removed_parts,
                &cloned_applicable_parts,
            );

            if let Some(mut result) = result {
                result.insert(part, number_of_parts_in_combination);
                return Some(result);
            }
        }
    }

    None
}

fn get_parts_string(parts: &HashMap<&str, usize>) -> String {
    // Build a string from the first letter of each part where each part is so
    // Each letter is repeated the number of times the part is used
    let mut keys: Vec<_> = parts.keys().map(|&k| k).collect();
    keys.sort();

    keys.iter()
        .map(|&part| part.chars().nth(0).unwrap().to_string().repeat(parts[part]))
        .collect()
}

fn solve_part_1(parts: &Parts) -> usize {
    // Return the part with the highest number of parts
    parts
        .keys()
        .map(|k| get_number_of_parts(k, parts))
        .max()
        .unwrap_or(0)
}

fn solve_part_2(parts: &Parts, missing_items: usize) -> String {
    let applicable_parts = get_applicable_parts(parts);

    let parts_combinations =
        find_combination_of_missing_parts(20, missing_items, &applicable_parts).unwrap();

    get_parts_string(&parts_combinations)
}

fn main() {
    let (missing_items, parts) = parse_input();
    println!("Part 1: {}", solve_part_1(&parts));
    println!("Part 2: {}", solve_part_2(&parts, missing_items));
}
