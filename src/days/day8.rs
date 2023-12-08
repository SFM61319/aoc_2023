use ahash::AHashMap;

#[aoc_runner_derive::aoc(day8, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().bytes().cycle();

    _ = lines.next();
    let mut adjacency_list = AHashMap::<&str, (&str, &str)>::new();

    for line in lines {
        let (from_node, to_nodes) = line.split_once('=').unwrap();

        let from_node = from_node.trim();
        let to_nodes = to_nodes.trim();

        let to_nodes = &to_nodes[1..(to_nodes.len() - 1)];
        let (left_node, right_node) = to_nodes.split_once(',').unwrap();

        let left_node = left_node.trim();
        let right_node = right_node.trim();

        adjacency_list.insert(from_node, (left_node, right_node));
    }

    let mut curr_node = "AAA";
    let mut steps = 0;

    loop {
        if curr_node == "ZZZ" {
            break steps;
        }

        steps += 1;
        curr_node = if directions.next().unwrap() == b'L' {
            adjacency_list[curr_node].0
        } else {
            adjacency_list[curr_node].1
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(super::solve_part1(input), 2)
    }

    #[test]
    fn test_solve_part1_sample_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(super::solve_part1(input), 6)
    }
}
