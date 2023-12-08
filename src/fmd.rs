//! Trouver la forme disjonctive

use crate::blocks::BlocksFound;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum GrayCodeValue {
    Change,
    Static(i32),
    #[default]
    None
}

pub(crate) fn get_disjonctive_form(
    blocks_found: &BlocksFound,
    gray_code: &[String],
    var_number: usize
) -> String
{
    let mut expr: Vec<String> = Vec::new();

    for rectangle in blocks_found.ones.iter() {
        let mut var = vec![GrayCodeValue::default(); var_number];
        let vars = &mut var[..];

        for (x, y) in rectangle.elements.clone() {
            let gray_x = gray_code.get(x).unwrap();
            let gray_y = gray_code.get(y).unwrap();

            let full_gray_code = format!("{gray_x}{gray_y}").chars().collect::<Vec<char>>();

            #[allow(clippy::needless_range_loop)]
            for i in 0..full_gray_code.len() {
                let graycode_for_position_i = full_gray_code.get(i).unwrap().to_string().parse().unwrap();

                match &mut vars[i] {
                    GrayCodeValue::None => vars[i] = GrayCodeValue::Static(graycode_for_position_i),
                    GrayCodeValue::Static(value) => {
                        if *value != graycode_for_position_i {
                            vars[i] = GrayCodeValue::Change;
                        }
                    },
                    _ => {}
                }
            }
        }

        // filter only variables that are static, the one we needs
        let cloned_vars = vars.to_vec().clone();
        let final_vars = cloned_vars.iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, GrayCodeValue::Static(_)))
            .collect::<Vec<(usize, &GrayCodeValue)>>();

        if final_vars.is_empty() {
            continue
        }

        // construct the code
        let mut sub_expr = String::new();
        for (i, var) in final_vars {
            let c = if let GrayCodeValue::Static(i) = var { i } else { continue };
            if c == &1 {
                sub_expr.push(LETTERS[i])
            } else {
                sub_expr.push_str(BARRED_LETTERS[i])
            }
        }

        expr.push(sub_expr);
    }

    expr.join(" + ")
}

const LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const BARRED_LETTERS: [&str; 26] = ["ā", "!b", "!c", "!d", "!e", "!f", "!g", "!h", "!i", "!j", "!k", "!l", "!m", "!n", "ō", "!p", "!q", "!r", "!s", "!t", "ū", "!v", "!w", "x̄", "!y", "!z"];
