use std::fs;

fn update_guard_loc(map: &mut Vec<Vec<char>>, guard_loc: &mut (i32, i32)) {
    match map[guard_loc.0 as usize][guard_loc.1 as usize] {
	'^' => {
	    if map[(guard_loc.0 - 1) as usize][guard_loc.1 as usize] == '#' {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = '>'
	    } else {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = 'X';
		guard_loc.0 -= 1;
		map[guard_loc.0 as usize][guard_loc.1 as usize] = '^';
	    }
	},
	'>' => {
	    if map[guard_loc.0 as usize][(guard_loc.1 + 1) as usize] == '#' {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = 'v'
	    } else {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = 'X';
		guard_loc.1 += 1;
		map[guard_loc.0 as usize][guard_loc.1 as usize] = '>';
	    }
	},
	'v' => {
	    if map[(guard_loc.0 + 1) as usize][guard_loc.1 as usize] == '#' {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = '<'
	    } else {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = 'X';
		guard_loc.0 += 1;
		map[guard_loc.0 as usize][guard_loc.1 as usize] = 'v';
	    }
	},
	'<' => {
	    if map[guard_loc.0 as usize][(guard_loc.1 - 1) as usize] == '#' {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = '^'
	    } else {
		map[guard_loc.0 as usize][guard_loc.1 as usize] = 'X';
		guard_loc.1 -= 1;
		map[guard_loc.0 as usize][guard_loc.1 as usize] = '<';
	    }
	},

	_ => println!("unreachable"),
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have able to read the file");
    let mut map: Vec<Vec<char>> = Vec::new();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    for line in lines {
	if line.len() == 0 {
	    // ignore empty line
	    break;
	}
	map.push(line.chars().collect::<Vec<char>>());
    }
    // Getting the first location of guard
    let mut height: i32 = 0;
    let mut width: i32 = 0;
    let mut guard_loc: (i32, i32) = (0, 0);
    for (i, lines) in map.iter().enumerate() {
	for (j, chr) in lines.iter().enumerate() {
	    if *chr == '^' {
		guard_loc.0 = i as i32;
		guard_loc.1 = j as i32;
	    }
	    height = (i + 1) as i32;
	    width = (j + 1) as i32;
	}
    }
    loop {
	update_guard_loc(&mut map, &mut guard_loc);
	match map[guard_loc.0 as usize][guard_loc.1 as usize] {
	    '^' => {
		if guard_loc.0 - 1 < 0 {
		    break;
		}
	    },
	    'v' => {
		if guard_loc.0 + 1 > height - 1 {
		    break;
		}
	    },
	    '>' => {
		if guard_loc.1 + 1 > width - 1 {
		    break;
		}
	    },
	    '<' => {
		if guard_loc.1 - 1 < 0 {
		    break;
		}
	    },
	    _ => println!("ureachable"),
	}
    }

    for line in map.iter() {
	println!("{:?}", line);
    }

    let mut count = 0;
    for line in map.iter() {
	for c in line.iter() {
	    if *c == 'X' {
		count += 1;
	    }
	}
    }
    println!("count: {}", count + 1);
}
