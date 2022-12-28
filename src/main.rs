mod utils;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1:");
    let day1_test_input = day1::common::read("data/testInput1")?;
    let day1_input = day1::common::read("data/input1")?;
    println!("part A test: {}", day1::a::solve(&day1_test_input));
    println!("part A: {}", day1::a::solve(&day1_input));
    println!("part B test: {}", day1::b::solve(&day1_test_input));
    println!("part B: {}", day1::b::solve(&day1_input));
    println!("Day 2:");
    let day2_test_input = day2::common::read("data/testInput2")?;
    let day2_input = day2::common::read("data/input2")?;
    println!("part A test: {}", day2::a::solve(&day2_test_input));
    println!("part A: {}", day2::a::solve(&day2_input));
    println!("part B test: {}", day2::b::solve(&day2_test_input));
    println!("part B: {}", day2::b::solve(&day2_input));
    println!("Day 3:");
    let day3_test_input = day3::common::read("data/testInput3")?;
    let day3_input = day3::common::read("data/input3")?;
    println!("part A test: {}", day3::a::solve(&day3_test_input));
    println!("part A: {}", day3::a::solve(&day3_input));
    println!("part B test: {}", day3::b::solve(&day3_test_input));
    println!("part B: {}", day3::b::solve(&day3_input));
    println!("Day 4:");
    let day4_test_input = day4::common::read("data/testInput4")?;
    let day4_input = day4::common::read("data/input4")?;
    println!("part A test: {}", day4::a::solve(&day4_test_input));
    println!("part A: {}", day4::a::solve(&day4_input));
    println!("part B test: {}", day4::b::solve(&day4_test_input));
    println!("part B: {}", day4::b::solve(&day4_input));
    println!("Day 5:");
    let day5_test_input = day5::common::read("data/testInput5")?;
    let day5_input = day5::common::read("data/input5")?;
    println!("part A test: {}", day5::a::solve(&day5_test_input));
    println!("part A: {}", day5::a::solve(&day5_input));
    println!("part B test: {}", day5::b::solve(&day5_test_input));
    println!("part B: {}", day5::b::solve(&day5_input));
    println!("Day 6:");
    let day6_test_input_1 = day6::common::read("data/testInput6_1")?;
    let day6_test_input_2 = day6::common::read("data/testInput6_2")?;
    let day6_test_input_3 = day6::common::read("data/testInput6_3")?;
    let day6_test_input_4 = day6::common::read("data/testInput6_4")?;
    let day6_test_input_5 = day6::common::read("data/testInput6_5")?;
    let day6_input = day6::common::read("data/input6")?;
    println!("part A test 1: {}", day6::a::solve(&day6_test_input_1));
    println!("part A test 2: {}", day6::a::solve(&day6_test_input_2));
    println!("part A test 3: {}", day6::a::solve(&day6_test_input_3));
    println!("part A test 4: {}", day6::a::solve(&day6_test_input_4));
    println!("part A test 5: {}", day6::a::solve(&day6_test_input_5));
    println!("part A: {}", day6::a::solve(&day6_input));
    println!("part B test 1: {}", day6::b::solve(&day6_test_input_1));
    println!("part B test 2: {}", day6::b::solve(&day6_test_input_2));
    println!("part B test 3: {}", day6::b::solve(&day6_test_input_3));
    println!("part B test 4: {}", day6::b::solve(&day6_test_input_4));
    println!("part B test 5: {}", day6::b::solve(&day6_test_input_5));
    println!("part B: {}", day6::b::solve(&day6_input));
    println!("Day 7:");
    let day7_test_input = day7::common::read("data/testInput7")?;
    let day7_input = day7::common::read("data/input7")?;
    println!("part A test: {}", day7::a::solve(&day7_test_input));
    println!("part A: {}", day7::a::solve(&day7_input));
    println!("part B test: {}", day7::b::solve(&day7_test_input));
    println!("part B: {}", day7::b::solve(&day7_input));
    println!("Day 8:");
    let day8_test_input = day8::common::read("data/testInput8")?;
    let day8_input = day8::common::read("data/input8")?;
    println!("part A test: {}", day8::a::solve(&day8_test_input));
    println!("part A: {}", day8::a::solve(&day8_input));
    println!("part B test: {}", day8::b::solve(&day8_test_input));
    println!("part B: {}", day8::b::solve(&day8_input));
    println!("Day 9:");
    let day9_test_input_1 = day9::common::read("data/testInput9_1")?;
    let day9_test_input_2 = day9::common::read("data/testInput9_2")?;
    let day9_input = day9::common::read("data/input9")?;
    println!("part A test 1: {}", day9::a::solve(&day9_test_input_1));
    println!("part A test 2: {}", day9::a::solve(&day9_test_input_2));
    println!("part A: {}", day9::a::solve(&day9_input));
    println!("part B test 1: {}", day9::b::solve(&day9_test_input_1));
    println!("part B test 2: {}", day9::b::solve(&day9_test_input_2));
    println!("part B: {}", day9::b::solve(&day9_input));
    println!("Day 10:");
    let day10_test_input = day10::common::read("data/testInput10")?;
    let day10_input = day10::common::read("data/input10")?;
    println!("part A test: {}", day10::a::solve(&day10_test_input));
    println!("part A: {}", day10::a::solve(&day10_input));
    println!("part B test: {}", day10::b::solve(&day10_test_input));
    println!("part B: {}", day10::b::solve(&day10_input));
    println!("Day 11:");
    let day11_test_input = day11::common::read("data/testInput11")?;
    let day11_input = day11::common::read("data/input11")?;
    println!("part A test: {}", day11::a::solve(&day11_test_input));
    println!("part A: {}", day11::a::solve(&day11_input));
    println!("part B test: {}", day11::b::solve(&day11_test_input));
    println!("part B: {}", day11::b::solve(&day11_input));
    println!("Day 12:");
    let day12_test_input = day12::common::read("data/testInput12")?;
    let day12_input = day12::common::read("data/input12")?;
    println!("part A test: {}", day12::a::solve(&day12_test_input));
    println!("part A: {}", day12::a::solve(&day12_input));
    println!("part B test: {}", day12::b::solve(&day12_test_input));
    println!("part B: {}", day12::b::solve(&day12_input));
    println!("Day 13:");
    let day13_test_input = day13::common::read("data/testInput13")?;
    let day13_input = day13::common::read("data/input13")?;
    println!("part A test: {}", day13::a::solve(&day13_test_input));
    println!("part A: {}", day13::a::solve(&day13_input));
    println!("part B test: {}", day13::b::solve(&day13_test_input));
    println!("part B: {}", day13::b::solve(&day13_input));
    println!("Day 14:");
    let day14_test_input = day14::common::read("data/testInput14")?;
    let day14_input = day14::common::read("data/input14")?;
    println!("part A test: {}", day14::a::solve(&day14_test_input));
    println!("part A: {}", day14::a::solve(&day14_input));
    println!("part B test: {}", day14::b::solve(&day14_test_input));
    println!("part B: {}", day14::b::solve(&day14_input));
    println!("Day 15:");
    let day15_test_input = day15::common::read("data/testInput15")?;
    let day15_input = day15::common::read("data/input15")?;
    println!("part A test: {}", day15::a::solve(&day15_test_input, 10));
    println!("part A: {}", day15::a::solve(&day15_input, 2000000));
    println!("part B test: {}", day15::b::solve(&day15_test_input, 20));
    println!("part B: {}", day15::b::solve(&day15_input, 4000000));
    println!("Day 16:");
    let day16_test_input = day16::common::read("data/testInput16")?;
    let day16_input = day16::common::read("data/input16")?;
    println!("part A test: {}", day16::a::solve(&day16_test_input));
    println!("part A: {}", day16::a::solve(&day16_input));
    println!("part B test: {}", day16::b::solve(&day16_test_input));
    println!("part B: {}", day16::b::solve(&day16_input));
    println!("Day 17:");
    let day17_test_input = day17::common::read("data/testInput17")?;
    let day17_input = day17::common::read("data/input17")?;
    println!("part A test: {}", day17::a::solve(&day17_test_input));
    println!("part A: {}", day17::a::solve(&day17_input));
    println!("part B test: {}", day17::b::solve(&day17_test_input));
    println!("part B: {}", day17::b::solve(&day17_input));
    println!("Day 18:");
    let day18_test_input = day18::common::read("data/testInput18")?;
    let day18_input = day18::common::read("data/input18")?;
    println!("part A test: {}", day18::a::solve(&day18_test_input));
    println!("part A: {}", day18::a::solve(&day18_input));
    println!("part B test: {}", day18::b::solve(&day18_test_input));
    println!("part B: {}", day18::b::solve(&day18_input));
    println!("Day 19:");
    let day19_test_input = day19::common::read("data/testInput19")?;
    let day19_input = day19::common::read("data/input19")?;
    println!("part A test: {}", day19::a::solve(&day19_test_input));
    println!("part A: {}", day19::a::solve(&day19_input));
    println!("part B test: {}", day19::b::solve(&day19_test_input));
    println!("part B: {}", day19::b::solve(&day19_input));
    println!("Day 20:");
    let day20_test_input = day20::common::read("data/testInput20")?;
    let day20_input = day20::common::read("data/input20")?;
    println!("part A test: {}", day20::a::solve(&day20_test_input));
    println!("part A: {}", day20::a::solve(&day20_input));
    println!("part B test: {}", day20::b::solve(&day20_test_input));
    println!("part B: {}", day20::b::solve(&day20_input));
    println!("Day 21:");
    let day21_test_input = day21::common::read("data/testInput21")?;
    let day21_input = day21::common::read("data/input21")?;
    println!("part A test: {}", day21::a::solve(&day21_test_input));
    println!("part A: {}", day21::a::solve(&day21_input));
    println!("part B test: {}", day21::b::solve(&day21_test_input));
    println!("part B: {}", day21::b::solve(&day21_input));
    println!("Day 22:");
    let day22_test_input = day22::common::read("data/testInput22")?;
    let day22_input = day22::common::read("data/input22")?;
    println!("part A test: {}", day22::a::solve(&day22_test_input));
    println!("part A: {}", day22::a::solve(&day22_input));
    println!("part B test: {}", day22::b::solve(&day22_test_input));
    println!("part B: {}", day22::b::solve(&day22_input));
    println!("Day 23:");
    let day23_test_input_1 = day23::common::read("data/testInput23_1")?;
    let day23_test_input_2 = day23::common::read("data/testInput23_2")?;
    let day23_input = day23::common::read("data/input23")?;
    println!("part A test: {}", day23::a::solve(&day23_test_input_1));
    println!("part A test: {}", day23::a::solve(&day23_test_input_2));
    println!("part A: {}", day23::a::solve(&day23_input));
    println!("part B test: {}", day23::b::solve(&day23_test_input_1));
    println!("part B test: {}", day23::b::solve(&day23_test_input_2));
    println!("part B: {}", day23::b::solve(&day23_input));
    println!("Day 24:");
    let day24_test_input = day24::common::read("data/testInput24")?;
    let day24_input = day24::common::read("data/input24")?;
    println!("part A test: {}", day24::a::solve(&day24_test_input));
    println!("part A: {}", day24::a::solve(&day24_input));
    println!("part B test: {}", day24::b::solve(&day24_test_input));
    println!("part B: {}", day24::b::solve(&day24_input));
    println!("Day 25:");
    let day25_test_input = day25::common::read("data/testInput25")?;
    let day25_input = day25::common::read("data/input25")?;
    println!("part A test: {}", day25::a::solve(&day25_test_input));
    println!("part A: {}", day25::a::solve(&day25_input));
    Ok(())
}