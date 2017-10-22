/// To handle a sequence of image operations
///
/// It will parse input file in this format:
/// load img/text.png
/// edge
/// threshold 40
/// invert
/// save img/lines.png

use model_collection::{ImageCommand};

// --------------------- parse_one ---------------------

pub fn parse_one(macro_input_raw: &str) -> ImageCommand {
    let macro_input = macro_input_raw.trim();
    let index_pt = macro_input.find(' ');
    let string_len = macro_input.len();
    let res = match index_pt {
        Some(index) => ImageCommand {
            command: &macro_input[0..index].trim(),
            parameter: &macro_input[index..string_len].trim(),
        },
        None => ImageCommand {
            command: macro_input.trim(),
            parameter: "",
        },
    };
    res
}

// --------------------- parse_one test ---------------------

#[test]
fn parse_one_1_test() {
    let expected = ImageCommand {
        command: "edge",
        parameter: "",
    };
    let result_found = parse_one("edge");
    assert_eq!(expected, result_found)
}

#[test]
fn parse_one_2_test() {
    let expected = ImageCommand {
        command: "threshold",
        parameter: "127",
    };
    let result_found = parse_one("threshold 127");
    assert_eq!(expected, result_found)
}

// --------------------- parse_all ---------------------

#[allow(dead_code)]
pub fn parse_all(macro_input: &str) -> Vec<ImageCommand> {
    let lines: Vec<&str> = macro_input.split(';').collect();
    let res: Vec<ImageCommand> = lines
        .iter()
        .filter(|line_e| 0 < line_e.trim().len())
        .map(|line| parse_one(line))
        .collect();
    res
}

// --------------------- parse_all test ---------------------

#[test]
fn parse_all_1_test() {
    let expected = ImageCommand {
        command: "edge",
        parameter: "",
    };
    let result_found = parse_all("edge");
    assert_eq!(expected, result_found[0]);
    assert_eq!(1, result_found.len())
}

#[test]
fn parse_all_2_1_test() {
    let expected = ImageCommand {
        command: "threshold",
        parameter: "127",
    };
    let result_found = parse_all("threshold 127");
    assert_eq!(expected, result_found[0]);
    assert_eq!(1, result_found.len());
    assert_eq!(vec![expected], result_found)
}

#[test]
fn parse_all_2_2_test() {
    let expected = ImageCommand {
        command: "threshold",
        parameter: "127",
    };
    let expected2 = ImageCommand {
        command: "gray",
        parameter: "",
    };
    let result_found = parse_all("threshold 127; gray");
    assert_eq!(expected, result_found[0]);
    assert_eq!(2, result_found.len());
    assert_eq!(vec![expected, expected2], result_found)
}

#[test]
fn parse_all_2_3_test() {
    let expected = ImageCommand {
        command: "threshold",
        parameter: "127",
    };
    let expected2 = ImageCommand {
        command: "gray",
        parameter: "",
    };
    let result_found = parse_all("threshold 127; gray;");
    assert_eq!(expected, result_found[0]);
    assert_eq!(2, result_found.len());
    assert_eq!(vec![expected, expected2], result_found)
}
