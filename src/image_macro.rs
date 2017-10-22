/// To handle a sequence of image operations
///
/// It will parse input file in this format:
/// load img/text.png
/// edge
/// threshold 40
/// invert
/// save img/lines.png

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageCommand<'a> {
    command: &'a str,
    parameter: &'a str,
}

pub fn parse_one(macro_input: &str) -> ImageCommand {
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
