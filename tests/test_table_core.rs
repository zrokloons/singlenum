use singlenum::components::table::core::Table;

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ...
 */
#[test]
fn test_01_engine_box_one_left() {
    let configuration: Vec<usize> = [
        8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 0, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.update_abox().unwrap();

    let result = table.engine_box_one_left().unwrap();
    assert!(result);
    assert_eq!(table.squares[18].value, 1_usize);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ...
 *
 */
#[test]
fn test_02_engine_box_one_left() {
    let configuration: Vec<usize> = [
        8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.update_abox().unwrap();

    let result = table.engine_box_one_left().unwrap();
    assert!(!result);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║   │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │   │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ...
 *
 */
#[test]
fn test_01_engine_line_one_left() {
    let configuration: Vec<usize> = [
        0, 5, 9, 6, 1, 2, 4, 3, 7, 0, 0, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.update_line().unwrap();

    let result = table.engine_line_one_left().unwrap();
    assert!(result);
    assert_eq!(table.squares[0].value, 8_usize);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║   │   │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ...
 *
 */
#[test]
fn test_02_engine_line_one_left() {
    let configuration: Vec<usize> = [
        0, 0, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.update_line().unwrap();

    let result = table.engine_line_one_left().unwrap();
    assert!(!result);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 3 │ 7 │ 5 ║ 2 │ 6 │ 8 ║ 9 │ 1 │ 4 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
 * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
 *
 */
#[test]
fn test_01_engine_column_one_left() {
    let configuration: Vec<usize> = [
        8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 0, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.update_column().unwrap();

    let result = table.engine_column_one_left().unwrap();
    assert!(result);
    assert_eq!(table.squares[18].value, 1_usize);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║   │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 3 │ 7 │ 5 ║ 2 │ 6 │ 8 ║ 9 │ 1 │ 4 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
 * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
 *
 */
#[test]
fn test_02_engine_column_one_left() {
    let configuration: Vec<usize> = [
        0, 5, 9, 6, 1, 2, 4, 3, 7, 0, 2, 3, 8, 5, 4, 1, 6, 9, 0, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.update_column().unwrap();

    let result = table.engine_column_one_left().unwrap();
    assert!(!result);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 3 │ 7 │ 5 ║ 2 │   │ 8 ║ 9 │ 1 │ 4 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
 * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
 *
 */
#[test]
fn test_01_engine_only_one_possible() {
    let configuration: Vec<usize> = [
        8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 0, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.squares[40].potentials = [6].to_vec();

    let result = table.engine_only_one_possible().unwrap();
    assert!(result);
    assert_eq!(table.squares[40].value, 6_usize);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 3 │ 7 │ 5 ║   │   │ 8 ║ 9 │ 1 │ 4 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
 * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
 *
 */
#[test]
fn test_02_engine_only_one_possible() {
    let configuration: Vec<usize> = [
        8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6,
        1, 4, 7, 3, 5, 2, 3, 7, 5, 0, 0, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1,
        6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);
    table.squares[39].potentials = [6, 2].to_vec();
    table.squares[40].potentials = [6, 2].to_vec();

    let result = table.engine_only_one_possible().unwrap();
    assert!(!result);
}

/*
 * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 * ║ 3 │   │   ║   │   │ 1 ║   │   │   ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │ 7 │ 1 ║ 9 │ 6 │   ║   │ 2 │ 4 ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │   │   ║ 5 │   │   ║   │   │ 1 ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║   │ 2 │   ║ 8 │ 4 │   ║ 7 │   │   ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │   │   ║ 6 │   │ 9 ║   │   │   ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │   │ 5 ║   │ 1 │ 2 ║   │ 9 │   ║
 * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 * ║ 9 │   │   ║   │   │ 6 ║   │   │   ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║ 2 │ 6 │   ║   │ 9 │ 7 ║ 1 │ 5 │   ║
 * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
 * ║   │   │   ║ 1 │   │   ║   │   │ 2 ║
 * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
 *
 *  ID: 27 potentials: [6, 1]
 *  ID: 28 potentials: []
 *  ID: 29 potentials: [3, 6, 9]
 *  ID: 36 potentials: [8, 1, 4, 7]
 *  ID: 37 potentials: [1, 3, 8, 4]
 *  ID: 38 potentials: [3, 4, 7, 8]
 *  ID: 45 potentials: [8, 6, 4, 7]
 *  ID: 46 potentials: [8, 3, 4]
 *  ID: 47 potentials: []
 *
 *  In this case it's only square 29 that have a potential for number 9
 *
 */
#[test]
fn test_01_engine_box() {
    let configuration: Vec<usize> = [
        3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 7, 1, 9, 6, 0, 0, 2, 4, 0, 0, 0, 5, 0, 0, 0, 0, 1, 0, 2, 0,
        8, 4, 0, 7, 0, 0, 0, 0, 0, 6, 0, 9, 0, 0, 0, 0, 0, 5, 0, 1, 2, 0, 9, 0, 9, 0, 0, 0, 0, 6,
        0, 0, 0, 2, 6, 0, 0, 9, 7, 1, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2,
    ]
    .to_vec();
    let mut table = Table::new(configuration, 1);

    table.update().unwrap();
    let result = table.engine_box().unwrap();
    assert!(result);
    assert_eq!(table.squares[29].value, 9_usize);
}
