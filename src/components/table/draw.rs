use crate::components::table;

/*
 * Draw the Table
 */
pub fn draw_table(table: &table::core::Table) {
    print!("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");
    let mut c = 0;
    for i in 0..81_usize {
        match i {
            27 | 54 => {
                println!();
                print!("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣");
            }
            9 | 18 | 36 | 45 | 63 | 72 => {
                println!();
                print!("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢");
            }
            _ => (),
        }
        if i % 9 == 0 {
            println!();
            print!("║"); // beginning
            c = 0;
        };

        let value = table.squares[i].value;
        match value {
            0 => print!("   "),
            _ => print!(" {value} "),
        }
        if c == 2 || c == 5 || c == 8 {
            print!("║");
        } else {
            print!("│");
        }
        c += 1;
    }
    println!();
    println!("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
}
