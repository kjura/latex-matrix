use std::{io::{self, BufRead}, iter::zip};



struct UserInput {
    matrix_type: String,
    rows: String,
    cols: String
}

// impl UserInput {
//     fn extract_user_input(stdin: Stdin) {

//     }
// }

fn validate_matrix_type(user_input: &String, types_container: Vec<&str>) {
    if !types_container.contains(&user_input.as_str()) {
        println!("{} not in allowed matrix types. Allowed matrix types: {:?}. Quitting...", user_input, types_container);
        std::process::exit(0)
    }

}


fn validate_row_input_erros(matrix_errors: Vec<usize>, cols: u32) {
    if !matrix_errors.is_empty() {
        for err in matrix_errors {
            println!("Invalid number of elements in row {}, should be {}", err, cols)
        }
        return;
    }
}

fn main() {



    let types_container = vec![
        "matrix", "pmatrix", "bmatrix", "Bmatrix", "vmatrix", "Vmatrix"
    ];

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    println!(
        "Welcome to latex-matrix generator. Please input your matrix type. Possible choices {:?}:",
        types_container
    );
    
    let matrix_type = iterator.next().unwrap().unwrap();
    println!("Please specify number of rows...");
    let rows = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();
    println!("Please specify number of columns...");
    let cols = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();
    println!("{}", matrix_type);
    validate_matrix_type(&matrix_type, types_container);
    println!("You have chosen the following configuration: matrix_type={}, dimensions={}x{} ", matrix_type, rows, cols);
    

    println!("Please provide matrix content. Elements should be seperated by spaces per row...");
    let mut row_element_holder = Vec::<String>::new();
    for row_count in 0..rows as u64 {
        println!("Row number {}", row_count + 1);
        let temp_row = iterator.next().unwrap().unwrap();
        // println!("{}", temp_row)
        row_element_holder.push(temp_row);
    }
    

    let rows_container: Vec::<Vec<String>> = row_element_holder
    .iter()
    .map(
        |element_row| element_row.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>()
    ).collect();
    

    let matrix_errors: Vec<usize> = rows_container
    .iter()
    .enumerate()
    .filter(|(_, full_row)| full_row.len() != cols.try_into().unwrap())
    .map(|(row_count, _)| row_count + 1)
    .collect();

    validate_row_input_erros(matrix_errors, cols);


    let begin_mark = "$ \\begin{".to_owned() + &matrix_type + "}";
    let end_mark = "\\end{".to_owned() + &matrix_type + "} $";

    let delimiters_container = rows_container.iter().rev().skip(1);

    let mut delimiters_container = Vec::<String>::new();
    for _ in rows_container.iter().rev().skip(1).rev(){
        delimiters_container.push(" & ".to_string());
    }
    delimiters_container.push(r" \\ ".to_string());


    println!("{}", begin_mark);
    for row in rows_container {
        let mut temp_row_formatter = String::new();
        for (elem, delimit) in zip(row, delimiters_container.clone()) {
            temp_row_formatter += &(elem + &delimit.to_string());
            println!("{}", temp_row_formatter)
        }
    }
    println!("{}", end_mark);



    // "35 67 89" => "35 & 67 & 89"
    // vec![42, 56, 78] ==> "42 & 56 & 78"

    // $ \begin{bmatrix}
    // a_1 & a_2 & a_3 \\
    // b_1 & b_2 & b_3 \\
    // c_1 & c_2 & c_3 
    // \end{bmatrix}  $


    // $ \begin{bmatrix} a_1 & a_2 & a_3 \\ b_1 & b_2 & b_3 \\ c_1 & c_2 & c_3 \end{bmatrix}  $

}
