use array2d::Array2D;
use crate::utils::file_reader::read_lines;

const ROWS:usize = 5;
const COLUMNS:usize = 5; 

pub fn d10p1(){
    let (x,y,maze) = get_maze();
    println!("{} {}", x, y);
    let result = get_furtherest_point(x, y, maze);
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

fn get_furtherest_point(x:usize, y:usize, maze:Array2D<char>) -> u32 {
    let mut result:u32 = 1;
    let mut prev_x = x;
    let mut prev_y = y;

    let (mut curr_x, mut curr_y) = get_first_point(x, y, &maze);
    
    while maze.get(curr_y, curr_x).unwrap().to_string() != 'S'.to_string() {
        println!("{}\n{}{}\n{}{}", maze.get(curr_y, curr_x).unwrap(),prev_x, prev_y, curr_x, curr_y);
        let (next_x, next_y) = get_next_point(maze.get(curr_y, curr_x).unwrap(), prev_x, prev_y, curr_x, curr_y);
        println!("{}{}", next_x, next_y);
        prev_x = curr_x;
        prev_y = curr_y;
        curr_x = next_x;
        curr_y = next_y;
        result += 1;
    }

    result
}

fn get_first_point(x:usize, y:usize, maze:&Array2D<char>) -> (usize, usize) {
    if maze.get(y-1, x).unwrap().to_string() == "-" || maze.get(y-1, x).unwrap().to_string() == "L" || maze.get(y-1, x).unwrap().to_string() == "F" {
        return (x, y-1)
    } 
    (x-1,y)
}

fn get_next_point(curr_pipe:&char, prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    match *curr_pipe {
        '|' => {return handle_north_south(prev_x, prev_y, curr_x, curr_y)},
        '-' => {return handle_east_west(prev_x, prev_y, curr_x, curr_y)},
        'L' => {return handle_north_east(prev_x, prev_y, curr_x, curr_y)},
        'J' => {return handle_north_west(prev_x, prev_y, curr_x, curr_y)},
        'F' => {return handle_east_south(prev_x, prev_y, curr_x, curr_y)},
        '7' => {return handle_west_south(prev_x, prev_y, curr_x, curr_y)},
         _ => unimplemented!("Found unimplemented character: {}", curr_pipe),
    }
}

fn handle_north_south(prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    if prev_y < curr_y {
        return (curr_x, curr_y+1);
    }
    (curr_x,curr_y-1)
}

fn handle_east_west(prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    if prev_x < curr_x {
        return (curr_x + 1, curr_y)
    }
    (curr_x - 1, curr_y)
}

fn handle_north_east(prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    if prev_y == curr_y {
        return (curr_x, curr_y+1)
    }
    (curr_x+1, curr_y)
}

fn handle_north_west(prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    if prev_y == curr_y {
        return (curr_x, curr_y+1)
    }
    (curr_x-1, curr_y)
}

fn handle_east_south(prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    if prev_y == curr_y {
        return (curr_x, curr_y-1)
    }
    (curr_x+1, curr_y)
}

fn handle_west_south(prev_x:usize, prev_y:usize, curr_x:usize, curr_y:usize) -> (usize,usize) {
    if prev_y == curr_y {
        return (curr_x, curr_y-1)
    }
    (curr_x-1, curr_y)
}

