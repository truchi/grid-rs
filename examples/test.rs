use grid::*;

fn main() {
    // let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let vec = vec![
        'a', 'b', 'c', 'd', //
        'e', 'f', 'g', 'h', //
        'i', 'j', 'k', 'l', //
    ];
    let mut flat = RowFlat::new((4, 3).into(), vec).unwrap();
    println!("SIZE {:?}", flat.size());

    // ITEM
    print!("ITEM    : ");
    for y in 0..3 {
        for x in 0..4 {
            print!("({})", flat.item(Point { x, y }).unwrap());
        }
    }
    print!("\nITEM MUT: ");
    for y in 0..3 {
        for x in 0..4 {
            print!("({})", (&mut flat).item(Point { x, y }).unwrap());
        }
    }

    // ROW
    print!("\nROW     : ");
    for row in 0..3 {
        print!("({})", flat.row(row).unwrap().iter().collect::<String>());
    }
    print!("\nROW MUT : ");
    for row in 0..3 {
        print!(
            "({})",
            (&mut flat).row(row).unwrap().iter().collect::<String>()
        );
    }

    // COL
    print!("\nCOL     : ");
    for col in 0..4 {
        print!("({})", flat.col(col).unwrap().collect::<String>());
    }
    print!("\nCOL MUT : ");
    for col in 0..4 {
        print!(
            "({})",
            (&mut flat)
                .col(col)
                .unwrap()
                .map(|c| *c)
                .collect::<String>()
        );
    }

    // ROWS
    print!("\nROWS    : ");
    for row in flat.rows(()).unwrap() {
        print!("({})", row.iter().collect::<String>());
    }
    print!("\nROWS MUT: ");
    for row in (&mut flat).rows(()).unwrap() {
        print!("({})", row.iter().collect::<String>());
    }

    // COLS
    print!("\nCOLS    : ");
    for col in flat.cols(()).unwrap() {
        print!("({})", col.collect::<String>());
    }
    print!("\nCOLS MUT: ");
    for col in (&mut flat).cols(()).unwrap() {
        print!("({})", col.collect::<String>());
    }

    println!();
}
