use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
#[cfg(feature = "time_calc")]
use std::time::Instant;
use clap::Parser;

mod printer;
mod blocks;
mod fmd;
mod fmc;

type KarnaughDiagram<'a> = &'a [&'a [i32]];


/// Generates a Gray code sequence of length `n`.
///
/// # Arguments
///
/// * `n` - The length of the Gray code sequence to be generated.
///
/// # Returns
///
/// A vector of strings representing the Gray code sequence.
///
/// # Examples
///
/// ```
/// let gray_code = generate_gray_code(3);
/// assert_eq!(gray_code, vec!["000", "001", "011", "010", "110", "111", "101", "100"]);
/// ```
fn generate_gray_code(n: usize) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let prev_gray_code = generate_gray_code(n - 1);
    let mut gray_code = prev_gray_code.clone();

    for code in prev_gray_code.into_iter().rev() {
        gray_code.push(format!("1{}", code));
    }

    gray_code.iter_mut().for_each(|code| {
        if code.len() < n {
            *code = format!("0{}", code);
        }
    });

    gray_code
}

#[derive(Parser)]
struct App {
    #[clap(short, long)]
    diagram: PathBuf
}

fn parse_table_from_file(path: &PathBuf) -> Vec<Vec<i32>> {
    let f = fs::read_to_string(path).expect("Impossible d'ouvrir le fichier");

    let mut diagram = Vec::new();
    let mut columns = None;

    for line in f.split('\n') {
        let mut bools = Vec::new();
        for c in line.chars() {
            match c {
                '0' => bools.push(0),
                '1' => bools.push(1),
                _ => panic!("Invalid input: the diagram can only have 0 and 1")
            }
        }

        match columns {
            None => columns = Some(bools.len()),
            Some(c) if bools.len() != c => panic!("A row don't have the same length as the others"),
            _ => {}
        }

        diagram.push(bools)
    }

    dbg!(&diagram);

    diagram
}

fn main() {
    let app = App::parse();

    // let karnaugh_diagram: KarnaughDiagram = &[
    //     &[0, 1, 1, 0],
    //     &[1, 0, 0, 1],
    //     &[1, 0, 0, 1],
    //     &[0, 1, 1, 0]
    // ];
    // let number_of_vars = 4;
    let parsed_diagram = parse_table_from_file(&app.diagram);
    let karnaugh_diagram = parsed_diagram.iter()
        .map(|u| u.as_slice())
        .collect::<Vec<&[i32]>>();
    let karnaugh_diagram = karnaugh_diagram.as_slice();

    let number_of_vars = karnaugh_diagram.len();
    let gray_code = generate_gray_code(number_of_vars / 2);

    #[cfg(feature = "draw_table")]
    printer::print_table(&karnaugh_diagram, number_of_vars, &gray_code);

    #[cfg(feature = "verbose")]
    println!("Calcule en cours des blocs...");
    #[cfg(feature = "time_calc")]
    let n = Instant::now();
    let blocks = blocks::find_blocks(&karnaugh_diagram);
    #[cfg(feature = "time_calc")]
    println!("Calculs des blocs (of 0 & 1) effectué en {}s", n.elapsed().as_secs_f32());

    let disjonctive_form  = fmd::get_disjonctive_form(&blocks, &gray_code, number_of_vars);
    println!("Forme minimale disjonctive: {disjonctive_form}");
    let conjonctive_form  = fmc::get_conjonctive_form(&blocks, &gray_code, number_of_vars);
    println!("Forme minimale conjonctive: {conjonctive_form}");


    #[cfg(feature = "time_calc")]
    println!("Les formes disjonctives et conjonctives ont été déterminées en {}s", n.elapsed().as_secs_f32());
}

pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
