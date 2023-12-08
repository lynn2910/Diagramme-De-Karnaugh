use std::time::Instant;

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

fn main() {
    let karnaugh_table: KarnaughDiagram = &[
        &[0, 1, 1, 0],
        &[1, 0, 0, 1],
        &[1, 0, 0, 1],
        &[0, 1, 1, 0]
    ];
    let number_of_vars = 4;
    let gray_code = generate_gray_code(number_of_vars / 2);

    #[cfg(feature = "draw_table")]
    printer::print_table(&karnaugh_table, number_of_vars, &gray_code);

    #[cfg(feature = "verbose")]
    println!("Calcule en cours des blocs...");
    #[cfg(feature = "time_calc")]
    let n = Instant::now();
    let blocks = blocks::find_blocks(&karnaugh_table);
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
