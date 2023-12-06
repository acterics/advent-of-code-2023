pub mod task1 {

    use crate::transform::parse_transform;
    use crate::transform::Transform;
    use std::fs;

    pub fn solution(filename: &str) -> i64 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines = file_content.split('\n');

        let seeds_line = lines.next().unwrap();
        lines.next();

        let seeds = parse_seeds(&seeds_line);

        let mut transforms: Vec<Transform> = Vec::new();

        while let Some(transform_set) = parse_transform(&mut lines) {
            transforms.push(transform_set);
        }

        return seeds
            .into_iter()
            .map(|number| {
                transforms.iter().fold(number, |number, transform| {
                    transform.perform_transform(number)
                })
            })
            .min()
            .expect(&format!("Failed to get min value from results"));
    }

    fn parse_seeds(str: &str) -> Vec<i64> {
        let seeds_iterator = str
            .split(':')
            .nth(1)
            .expect(&format!("Failed to get seeds from {str}"))
            .trim()
            .split(' ')
            .map(|raw_number| {
                raw_number
                    .parse()
                    .expect(&format!("Failed to parse number ({raw_number}) from {str}"))
            });
        return Vec::from_iter(seeds_iterator);
    }
}

pub mod task2 {
    use crate::transform::parse_transform;
    use crate::transform::RangeWithOffset;
    use crate::transform::Transform;
    use std::cmp::min;
    use std::fs;

    pub fn solution(filename: &str) -> i64 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines = file_content.split('\n');

        let seeds_line = lines.next().unwrap();
        lines.next();

        let mut seed_ranges = parse_seed_ranges(&seeds_line);
        seed_ranges.sort_by_key(|range_with_offset| range_with_offset.range.start);

        let mut transforms: Vec<Transform> = Vec::new();

        while let Some(transform) = parse_transform(&mut lines) {
            transforms.push(transform);
        }

        let result_ranges = transforms.iter().fold(seed_ranges, |ranges, transform| {
            next_ranges_with_offset(&ranges, &transform)
        });

        let result = result_ranges.iter().min_by_key(|range_with_offset| {
            range_with_offset.range.start + range_with_offset.offset
        }).unwrap();

        return result.range.start + result.offset;
    }

    fn next_ranges_with_offset(
        ranges: &Vec<RangeWithOffset>,
        transform: &Transform,
    ) -> Vec<RangeWithOffset> {
        let next_ranges = ranges.iter().flat_map(|range_with_offset| {
            let offset = &range_with_offset.offset;
            let range = &range_with_offset.range;

            let mut current_range_start = range.start;
            let mut instructions_iterator = transform.instructions.iter();
            let mut current_instruction = instructions_iterator.next();
            let mut new_ranges: Vec<RangeWithOffset> = Vec::new();

            while current_range_start < range.end {
                let shifted_range_start = current_range_start + offset;
                let shifted_range_end = range.end + offset;
                match current_instruction {
                    Some(instruction) => {
                        let instruction_source_end = instruction.source_start + instruction.size;
                        let instruction_offset = instruction.dest_start - instruction.source_start;
                        if shifted_range_start < instruction.source_start {
                            if shifted_range_end < instruction_source_end {
                                let new_range = current_range_start..(range.end);
                                new_ranges.push(RangeWithOffset {
                                    range: new_range,
                                    offset: *offset,
                                });
                                break;
                            }
                            let distance = instruction.source_start - shifted_range_start;
                            let new_range = current_range_start..(current_range_start + distance);
                            new_ranges.push(RangeWithOffset {
                                range: new_range,
                                offset: *offset,
                            });
                            current_range_start += distance;
                            continue;
                        }
                        if shifted_range_start < instruction_source_end {
                            let new_range_end = min(shifted_range_end, instruction_source_end);
                            let distance = new_range_end - shifted_range_start;
                            let new_range = current_range_start..(current_range_start + distance);
                            new_ranges.push(RangeWithOffset {
                                range: new_range,
                                offset: offset + instruction_offset,
                            });
                            current_range_start += distance;
                            current_instruction = instructions_iterator.next();
                            continue;
                        }
                        current_instruction = instructions_iterator.next();
                    }
                    None => {
                        let new_range = current_range_start..range.end;
                        new_ranges.push(RangeWithOffset {
                            range: new_range,
                            offset: *offset,
                        });
                        break;
                    }
                }
            }
            return new_ranges;
        });
        return Vec::from_iter(next_ranges);
    }

    fn parse_seed_ranges(str: &str) -> Vec<RangeWithOffset> {
        let seeds_iterator = str
            .split(':')
            .nth(1)
            .expect(&format!("Failed to get seeds from {str}"))
            .trim()
            .split(' ')
            .map(|raw_number| -> i64 {
                raw_number
                    .parse()
                    .expect(&format!("Failed to parse number ({raw_number}) from {str}"))
            });
        let seeds_vector = Vec::from_iter(seeds_iterator);
        let ranges = seeds_vector.chunks(2).map(|chunk| RangeWithOffset {
            range: chunk[0]..(chunk[0] + chunk[1]),
            offset: 0,
        });
        return Vec::from_iter(ranges);
    }
}

mod transform {
    use std::fmt::Debug;
    use std::ops::Range;

    pub struct RangeWithOffset {
        pub range: Range<i64>,
        pub offset: i64,
    }

    impl Debug for RangeWithOffset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RangeWithOffset")
                .field("range", &self.range)
                .field("offset", &self.offset)
                .finish()
        }
    }

    pub struct Transform {
        pub name: String,
        pub instructions: Vec<TransformInstruction>,
    }

    pub struct TransformInstruction {
        pub dest_start: i64,
        pub source_start: i64,
        pub size: i64,
    }

    impl Transform {
        pub fn perform_transform(&self, number: i64) -> i64 {
            let target_instruction = self.instructions.iter().find(|transform| {
                number >= transform.source_start && number < transform.source_start + transform.size
            });
            let result: i64;
            if let Some(target_instruction) = target_instruction {
                result = target_instruction.dest_start + (number - target_instruction.source_start)
            } else {
                result = number
            }
            return result;
        }
    }

    pub fn parse_transform<'a, I>(lines: &mut I) -> Option<Transform>
    where
        I: Iterator<Item = &'a str>,
    {
        let transform_name = lines.next();
        if transform_name.is_none() {
            return None;
        }
        let transform_name = transform_name.unwrap();
        let mut instructions = Vec::new();
        while let Some(transform) = parse_transform_instruction(lines) {
            instructions.push(transform);
        }
        if instructions.is_empty() {
            return None;
        }
        instructions.sort_by_key(|instruction| instruction.source_start);
        return Some(Transform {
            name: String::from_iter(transform_name.chars()),
            instructions,
        });
    }

    pub fn parse_transform_instruction<'a, I>(lines: &mut I) -> Option<TransformInstruction>
    where
        I: Iterator<Item = &'a str>,
    {
        let transform_line = lines.next();
        if transform_line.is_none() {
            return None;
        }
        let transform_line = transform_line.unwrap().trim();
        if transform_line.is_empty() {
            return None;
        }

        let mut params = transform_line.split(' ').map(|raw_number| -> i64 {
            raw_number.parse().expect(&format!(
                "Failed to parse number {raw_number} from {transform_line}"
            ))
        });
        let dest_start = params
            .next()
            .expect(&format!("Cannot get dest_start from {transform_line}"));
        let source_start = params
            .next()
            .expect(&format!("Cannot get source_start from {transform_line}"));
        let size = params
            .next()
            .expect(&format!("Cannot get size from {transform_line}"));

        return Some(TransformInstruction {
            dest_start,
            source_start,
            size,
        });
    }
}
