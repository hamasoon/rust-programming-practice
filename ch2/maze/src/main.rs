use rand::Rng;

const MAZE_SIZE: usize = 20;
const WALL: [char; 2] = ['⬜', '⬛'];

fn main() {
    let mut _rng = rand::thread_rng();

    let mut maze = [[0; MAZE_SIZE]; MAZE_SIZE];

    for n in 0..MAZE_SIZE {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAZE_SIZE-1] = 1;
        maze[MAZE_SIZE-1][n] = 1;
    }

    for i in 1..MAZE_SIZE-1 {
        if i % 2 == 0 {
            continue;
        }

        for j in 1..MAZE_SIZE-1 {
            if j % 2 == 0 {
                continue;
            }

            maze[i][j] = 1;

            let r = _rng.gen_range(0..4);

            match r {
                0 => {
                    maze[i][j+1] = 1;
                },
                1 => {
                    maze[i+1][j] = 1;
                },
                2 => {
                    maze[i][j-1] = 1;
                },
                3 => {
                    maze[i-1][j] = 1;
                },
                _ => {}
            }
        }
    }

    for i in 0..MAZE_SIZE {
        for j in 0..MAZE_SIZE {
            print!("{}", WALL[maze[i][j]]);
        }
        println!();
    }
}
