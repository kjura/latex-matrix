use std::{io::{self, BufRead}, iter::zip};

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

    println!("You have chosen the following configuration: matrix_type={}, dimensions={}x{} ", matrix_type, rows, cols);
    
    if !types_container.contains(&matrix_type.as_str()) {
        println!("{} not in allowed matrix types. Allowed matrix types: {:?}. Quitting...", matrix_type, types_container);
        return
    }

    println!("Please provide matrix content. Elements should be seperated by spaces per row...");
    let mut row_element_holder = Vec::<String>::new();
    for row_count in 0..rows as u64 {
        println!("Row number {}", row_count + 1);
        let temp_row = iterator.next().unwrap().unwrap();
        // println!("{}", temp_row)
        row_element_holder.push(temp_row);
    }
    
    let mut rows_container = Vec::<Vec<String>>::new();
    for element_row in row_element_holder {
        let temp_row_split = element_row.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>();
        rows_container.push(temp_row_split);
    }

    let mut matrix_errors = Vec::<usize>::new();
    for (row_count, full_row) in rows_container.iter().enumerate() {
        if full_row.len() != cols.try_into().unwrap() {
            matrix_errors.push(row_count + 1)
        }
    }

    if !matrix_errors.is_empty() {
        for err in matrix_errors {
            println!("Invalid number of elements in row {}, should be {}", err, cols)
        }
        return;
    }


    let begin_mark = "$ \\begin{".to_owned() + &matrix_type + "}";
    let end_mark = "\\end{".to_owned() + &matrix_type + "} $";
    
    // for rows in rows_container {
    //     let mut formated_row = String::new();
    //     for elements in rows {
    //         formated_row += &elements.to_string() + " & "
    //     }
    // }

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


}
