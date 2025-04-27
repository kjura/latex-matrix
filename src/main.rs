use std::io::{self, BufRead};



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
    let last_row_of_data = rows_container.last().unwrap().clone().into_iter().rev().flat_map(|i| [i, String::from("&")]).rev().skip(1).collect::<Vec<String>>().join(" ");

    println!("{}", begin_mark);
    for row in rows_container.iter().rev().skip(1).rev() {

        let mut formatted_row: Vec<String> = 
        row
        .clone()
        .into_iter()
        .rev()
        .flat_map(|i| [i, String::from("&")])
        .rev()
        .skip(1)
        .collect();
        formatted_row.push("\\\\".to_string());
        println!("{}", formatted_row.join(" "))

    }
    println!("{}", last_row_of_data);
    println!("{}", end_mark);

}
