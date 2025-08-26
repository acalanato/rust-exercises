use rand::Rng;

fn _make_table() ->  [[u8;9];9] {
    let mut table:  [[u8;9];9] = [[0_u8;9];9];
    let random = |a,b| rand::thread_rng().gen_range(a..b);
    
    let mut i: usize = 0;
    for x in 0..9 {
	//for y in 0..3 {
	let k = random(0,9);
	if !table[i].contains(&k) {table[x][i] = k}
	i += 1;
	//}
    }    
    table
}

fn valid_row (input: [[u8;9];9], row: u8, n: u8) -> bool {
    for x in 0..9 {
	if input[row as usize][x] == n {
	    return true
	}  else {
	    return false
	}
    }
    false
}

fn valid_col (input: [[u8;9];9], col: u8, n: u8) -> bool {
    for x in 0..9 {
	if input[x][col as usize] == n {
	    return true
	}
    }
    false
}

fn valid_box (input: [[u8;9];9], row: u8, col: u8, n: u8) -> bool {
    for x in 0..3 {
	for y in 0..3 {
	    if input[x + row as usize][y + col as usize] == n {
		return true
	    }
	}
    }
    false
}


fn find_zero(input: [[u8;9];9], pos: (u8, u8)) -> bool {
    for x in 0..9 {
	
    }
    false
}

/*

function findEmptySpace(matrix, cell) {
  for (let i = 0; i < 9; i++) {
    for (let j = 0; j < 9; j++) {
      if (matrix[i][j] == 0) {
        cell[0] = i;
        cell[1] = j;
        return true;
      }
    }
  }
  return false;
}


*/


fn scamble_list() -> [u8;9] {
    let mut list = [1,2,3,4,5,6,7,8,9].map(|n| n as u8);

    //let mut table:  [[u8;9];9] = [[0_u8;9];9];
    
    let random = |a,b| rand::thread_rng().gen_range(a..b);
	
    for _x in 0..9 {
	let mut x = random(0,9);
	let mut y = random(0,9);
	
	'a: loop {
	    if x != y {
		break 'a;
	    } else {
		x = random(0,9);
		y = random(0,9);
	    }	    
	}
	
	let (a,b) = (list[x], list[y]);

	list[x] = b;
	list[y] = a;
    }
    
    list
}

fn fill_table(mut input: [[u8;9];9]) -> [[u8;9];9] {
    let first_row = scamble_list();
    let first_col = scamble_list();

    println!("{:?}\n{:?}", first_col, first_row);

    
    //first row
    for x in 0..9 {
	input[0][x] = first_row[x]
    }

    //first column
    for x in 1..9_usize {
	if first_row[0] == x as u8{
	    continue
	} else {
	    //input[x][0] = x as u8;
	    input[x][0] = first_col[x];
	    //continue
	}
    }
    
    input
}


fn _print_table(input: [[u8;9];9]) {
    
    let a = input[0];
    let b = input[1];
    let c = input[2];
    let d = input[3];
    let e = input[4];
    let f = input[5];
    let g = input[6];
    let h = input[7];
    let i = input[8];
   
    print!("
 {a0} {a1} {a2} | {a3} {a4} {a5} | {a6} {a7} {a8}
 {b0} {b1} {b2} | {b3} {b4} {b5} | {b6} {b7} {b8}
 {c0} {c1} {c2} | {c3} {c4} {c5} | {c6} {c7} {c8}
 - - - x - - - x - - -
 {d0} {d1} {d2} | {d3} {d4} {d5} | {d6} {d7} {d8}
 {e0} {e1} {e2} | {e3} {e4} {e5} | {e6} {e7} {e8}
 {f0} {f1} {f2} | {f3} {f4} {f5} | {f6} {f7} {f8}
 - - - x - - - x - - -
 {g0} {g1} {g2} | {g3} {g4} {g5} | {g6} {g7} {g8}
 {h0} {h1} {h2} | {h3} {h4} {h5} | {h6} {h7} {h8}
 {i0} {i1} {i2} | {i3} {i4} {i5} | {i6} {i7} {i8}

",
	   a0=a[0],  a1=a[1],  a2=a[2],  b0=b[0],  b1=b[1],  b2=b[2],  c0=c[0],  c1=c[1],  c2=c[2],
	   a3=a[3],  a4=a[4],  a5=a[5],  b3=b[3],  b4=b[4],  b5=b[5],  c3=c[3],  c4=c[4],  c5=c[5],
	   a6=a[6],  a7=a[7],  a8=a[8],  b6=b[6],  b7=b[7],  b8=b[8],  c6=c[6],  c7=c[7],  c8=c[8],
	   
	   d0=d[0],  d1=d[1],  d2=d[2],  e0=e[0],  e1=e[1],  e2=e[2],  f0=f[0],  f1=f[1],  f2=f[2],
	   d3=d[3],  d4=d[4],  d5=d[5],  e3=e[3],  e4=e[4],  e5=e[5],  f3=f[3],  f4=f[4],  f5=f[5],
	   d6=d[6],  d7=d[7],  d8=d[8],  e6=e[6],  e7=e[7],  e8=e[8],  f6=f[6],  f7=f[7],  f8=f[8],
	   
	   g0=g[0],  g1=g[1],  g2=g[2],  h0=h[0],  h1=h[1],  h2=h[2],  i0=i[0],  i1=i[1],  i2=i[2],
	   g3=g[3],  g4=g[4],  g5=g[5],  h3=h[3],  h4=h[4],  h5=h[5],  i3=i[3],  i4=i[4],  i5=i[5],
	   g6=g[6],  g7=g[7],  g8=g[8],  h6=h[6],  h7=h[7],  h8=h[8],  i6=i[6],  i7=i[7],  i8=i[8]

    );
}

fn main() {

    //let table = make_table();
    //print_table(table);

    //let table = scamble_list();
    //println!("{:?}", scamble_list());
    
    //println!("{:?}", valid_row([[2;9];9], 2, 2));


    let table = fill_table( [[0;9];9] );

    _print_table(table);
    
    println!("Done!");
}
