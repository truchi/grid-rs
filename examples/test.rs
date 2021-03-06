use grid::{grid1d::*, *};

fn main() {
    let vec = vec![
        'a', 'b', 'c', 'd', //
        'e', 'f', 'g', 'h', //
        'i', 'j', 'k', 'l', //
    ];
    let mut flat = RowGrid1D::new((4, 3).into(), vec).unwrap();
    println!("SIZE {:?}", flat.size());

    // ITEM
    print!("ITEM    : ");
    for y in 0..3 {
        for x in 0..4 {
            print!("({})", flat.item((x, y)).unwrap());
        }
    }
    print!("\nITEM MUT: ");
    for y in 0..3 {
        for x in 0..4 {
            print!("({})", (&mut flat).item((x, y)).unwrap());
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
    for row in flat.rows(..).unwrap() {
        print!("({})", row.iter().collect::<String>());
    }
    print!("\nROWS MUT: ");
    for row in (&mut flat).rows(..).unwrap() {
        print!("({})", row.iter().collect::<String>());
    }

    // COLS
    print!("\nCOLS    : ");
    for col in flat.cols(..).unwrap() {
        print!("({})", col.collect::<String>());
    }
    print!("\nCOLS MUT: ");
    for col in (&mut flat).cols(..).unwrap() {
        print!("({})", col.collect::<String>());
    }

    println!("\n=======================");

    // ITEM
    print!("ITEM    : ");
    for y in 0..3 {
        for x in 0..4 {
            print!("({})", flat.item((x, y)).unwrap());
        }
    }
    print!("\nITEM MUT: ");
    for y in 0..3 {
        for x in 0..4 {
            print!("({})", (&mut flat).item((x, y)).unwrap());
        }
    }

    // ROW
    print!("\nROW     : ");
    for row in 0..3 {
        print!(
            "({})",
            unsafe { flat.row_unchecked(row) }
                .iter()
                .collect::<String>()
        );
    }
    print!("\nROW MUT : ");
    for row in 0..3 {
        print!(
            "({})",
            unsafe { (&mut flat).row_unchecked(row) }
                .iter()
                .collect::<String>()
        );
    }

    // COL
    print!("\nCOL     : ");
    for col in 0..4 {
        print!(
            "({})",
            unsafe { flat.col_unchecked(col) }.collect::<String>()
        );
    }
    print!("\nCOL MUT : ");
    for col in 0..4 {
        print!(
            "({})",
            unsafe { (&mut flat).col_unchecked(col) }
                .map(|c| *c)
                .collect::<String>()
        );
    }

    // ROWS
    print!("\nROWS    : ");
    for row in flat.rows(..).unwrap() {
        print!("({})", row.iter().collect::<String>());
    }
    print!("\nROWS MUT: ");
    for row in (&mut flat).rows(..).unwrap() {
        print!("({})", row.iter().collect::<String>());
    }

    // COLS
    print!("\nCOLS    : ");
    for col in flat.cols(..).unwrap() {
        print!("({})", col.collect::<String>());
    }
    print!("\nCOLS MUT: ");
    for col in (&mut flat).cols(..).unwrap() {
        print!("({})", col.collect::<String>());
    }

    println!();
}
