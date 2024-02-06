// split by newline
// when double new line all previous get added to new dynamic vector
// dynamic vector shouold be on integers
// should have a method to compute the total of the integers

const TEST_STR: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

#[derive(Debug, Clone)]
struct Elf {
    index: u32,
    calories: Vec<u32>,
}
impl Elf {
    fn total_calories(&self) -> u32 {
        let mut calories = 0;
        for food_item in self.calories.as_slice() {
            calories += food_item;
        }
        calories
    }
}

fn elf_with_max_calories(
    elves: Vec<Elf>,
    // How many elves to report
    top_x: u32,
) {
    let mut sorted_elves: Vec<Elf> = elves.to_vec();
    sorted_elves.sort_by_key(|k| k.total_calories());
    let top_elves = &sorted_elves[sorted_elves.len() - top_x as usize..];

    println!("Reporting the top {top_x} maximum calories in descending order:");
    let mut total_calories: u32 = 0;
    for (idx, elf) in top_elves.iter().rev().enumerate() {
        println!(
            "{}",
            format!(
                "{}/{}: Elf #{} with calories {}",
                idx + 1,
                top_x,
                elf.index + 1,
                elf.total_calories(),
            )
        );
        total_calories += elf.total_calories();
    }
    println!("Total calories: {total_calories}")
}

fn comprehension_style(input_str: &str) {
    let input_list: Vec<&str> = input_str.split("\n\n").collect();
    let elves = input_list
        .iter()
        .map(|x: &&str| {
            {
                x.split("\n").map(
                    // we could technically use .filtermap() with .parse().ok()
                    |x: &str| match x.parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => 0,
                    },
                )
            }
            .collect::<Vec<u32>>()
        })
        .enumerate()
        .map(|(index, calories)| Elf {
            index: index as u32,
            calories: calories,
        })
        .collect::<Vec<Elf>>();

    elf_with_max_calories(elves, 3);
}

fn main() {
    // We could read the file as a string OR include which is a compile time check!
    // let input = std::fs::read_to_string(LOCAL_INPUT_FILE).unwrap();
    let input = include_str!("../../../../input/2022/d1.txt");
    // let input = TEST_STR;
    comprehension_style(&input)
}
