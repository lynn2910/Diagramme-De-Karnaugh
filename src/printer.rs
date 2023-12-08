use cli_table::{Cell, print_stdout, Style, Table};
use cli_table::format::{Border, HorizontalLine, Justify, Separator, VerticalLine};
use crate::{ALPHABET, KarnaughDiagram};

/// Prints a Karnaugh Table.
///
/// # Arguments
///
/// * `table` - A reference to the Karnaugh Table.
/// * `number_of_vars` - The number of variables in the table.
/// * `gray_code` - An array of gray codes associated with each row.
pub(crate) fn print_table(table: &KarnaughDiagram, number_of_vars: usize, gray_code: &[String]) {
    if number_of_vars % 2 != 0 {
        panic!("Un diagramme de karnaugh ne peut pas avoir un nombre non-pair de variables")
    }

    fn generate_vars_string(number_of_vars: usize) -> String {
        let middle = number_of_vars / 2;
        format!("{l}/{r}", r = &ALPHABET[0..middle], l = &ALPHABET[middle..number_of_vars])
    }

    let mut table_data = Vec::new();

    for (i, row) in table.iter().enumerate() {
        // add the gray code associated :)
        let mut r = vec![gray_code.get(i).unwrap_or(&"None".into()).cell().bold(true)];

        for elm in row.iter() {
            r.push((*elm as u8).to_string().cell().justify(Justify::Center));
        }

        table_data.push(r);
    }

    let mut title = vec![generate_vars_string(number_of_vars).cell().bold(true)];
    for code in gray_code.iter() {
        title.push(code.cell().bold(true))
    }

    let table = table_data.table()
        .title(title)
        .bold(true)
        .border(create_table_border())
        .separator(create_table_separator());

    println!("Diagramme de Karnaugh:");
    let _ = print_stdout(table);
}

fn create_table_border() -> Border {
    let vertical = VerticalLine::new('│');
    Border::builder()
        .bottom(
            HorizontalLine::new(
                '└',
                '┘',
                '┴',
                '─'
            )
        )
        .top(
            HorizontalLine::new(
                '┌',
                '┐',
                '┬',
                '─'
            )
        )
        .left(vertical)
        .right(vertical)
        .build()
}

fn create_table_separator() -> Separator {
    Separator::builder()
        .row(
            Some(
                HorizontalLine::new(
                    '├',
                    '┤',
                    '┼',
                    '─'
                )
            )
        )
        .column(
            Some(
                VerticalLine::new('│')
            )
        )
        .build()
}