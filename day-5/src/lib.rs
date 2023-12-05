pub mod task1 {

    use crate::transform::Transform;
    use crate::transform::TransformInstruction;
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
            .iter()
            .map(|number| {
                transforms.iter().fold(*number, |number, transform| {
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

    fn parse_transform<'a, I>(lines: &mut I) -> Option<Transform>
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
        return Some(Transform {
            name: String::from_iter(transform_name.chars()),
            instructions,
        });
    }

    fn parse_transform_instruction<'a, I>(lines: &mut I) -> Option<TransformInstruction>
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

mod transform {
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
}
