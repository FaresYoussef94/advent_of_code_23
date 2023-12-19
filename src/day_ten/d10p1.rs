use array2d::Array2D;
use crate::utils::file_reader::read_lines;

const ROWS:usize = 140;
const COLUMNS:usize = 140; 

pub fn d10p1(){
    let (x,y,maze) = get_maze();
    println!("{} {}", x, y);
    let loop_size = get_furtherest_point(x, y, maze);
    let result = loop_size/2 + loop_size%2;
    println!("D10P1 result: {}", result);
}

pub fn get_maze() -> (usize, usize, Array2D<char>) {
    let mut maze:Array2D<char> = Array2D::filled_with('.', ROWS, COLUMNS);
    let mut x:usize = 0;
    let mut y:usize = 0;

    if let Ok(lines) = read_lines("./resources/d10.txt") {
        let mut i:usize = 0;
        for line in lines {
            if let Ok(entry) = line {
                let mut j:usize = 0;
                for ch in entry.chars() {
                    if ch == 'S' {
                        x=i;
                        y=j;
                        println!("S: {} {}", x, y);
                    }
                    let _ = maze.set(i, j, ch);
                    j+=1;
                }
            }
        i+=1;
        }
    }

    (x, y, maze)
} 

fn get_furtherest_point(i:usize, j:usize, maze:Array2D<char>) -> u32 {
    let mut result:u32 = 1;
    let mut prev_i = i;
    let mut prev_j = j;

    let (mut curr_i, mut curr_j) = get_first_point(i, j, &maze);
    
    while maze.get(curr_i, curr_j).unwrap().to_string() != 'S'.to_string() {
        println!("current point:{}\nprev x,y: {}{}\ncurr x,y{}{}", maze.get(curr_i, curr_j).unwrap(),prev_i, prev_j, curr_i, curr_j);
        let (next_i, next_j) = get_next_point(maze.get(curr_i, curr_j).unwrap(), prev_i, prev_j, curr_i, curr_j);
        println!("next xy:{}{}", next_i, next_j);
        println!("next char:{}", maze.get(next_i, next_j).unwrap());
        prev_i = curr_i;
        prev_j = curr_j;
        curr_i = next_i;
        curr_j = next_j;
        result += 1;
    }

    result
}

fn get_first_point(i:usize, j:usize, maze:&Array2D<char>) -> (usize, usize) {
    if i > 0 && (maze.get(i-1, j).unwrap().to_string() == "|" || maze.get(i-1, j).unwrap().to_string() == "L" || maze.get(i-1, j).unwrap().to_string() == "J") {
        return (i-1, j);
    } else if i < ROWS - 1 && (maze.get(i+1, j).unwrap().to_string() == "|" || maze.get(i+1, j).unwrap().to_string() == "7" || maze.get(i+1, j).unwrap().to_string() == "F") {
        return (i+1, j);
    } else if j > 0 && (maze.get(i, j-1).unwrap().to_string() == "-" || maze.get(i, j-1).unwrap().to_string() == "L" || maze.get(i, j-1).unwrap().to_string() == "F") {
        return (i, j-1);
    }
    (i,j+1)
}

fn get_next_point(curr_pipe:&char, prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    match *curr_pipe {
        '|' => {return handle_north_south(prev_i, prev_j, curr_i, curr_j)},
        '-' => {return handle_east_west(prev_i, prev_j, curr_i, curr_j)},
        'L' => {return handle_north_east(prev_i, prev_j, curr_i, curr_j)},
        'J' => {return handle_north_west(prev_i, prev_j, curr_i, curr_j)},
        'F' => {return handle_east_south(prev_i, prev_j, curr_i, curr_j)},
        '7' => {return handle_west_south(prev_i, prev_j, curr_i, curr_j)},
         _ => unimplemented!("Found unimplemented character: {}", curr_pipe),
    }
}

fn handle_north_south(prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    if prev_i < curr_i {
        return (curr_i+1, curr_j);
    }
    (curr_i-1,curr_j)
}

fn handle_east_west(prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    if prev_j < curr_j {
        return (curr_i, curr_j + 1)
    }
    (curr_i, curr_j- 1)
}

fn handle_north_east(prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    if prev_i == curr_i {
        return (curr_i-1, curr_j)
    }
    (curr_i, curr_j+1)
}

fn handle_north_west(prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    if prev_i == curr_i {
        return (curr_i-1, curr_j)
    }
    (curr_i, curr_j-1)
}

fn handle_east_south(prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    if prev_i == curr_i {
        return (curr_i+1, curr_j)
    }
    (curr_i, curr_j+1)
}

fn handle_west_south(prev_i:usize, prev_j:usize, curr_i:usize, curr_j:usize) -> (usize,usize) {
    if prev_i == curr_i {
        return (curr_i+1, curr_j)
    }
    (curr_i, curr_j-1)
}

