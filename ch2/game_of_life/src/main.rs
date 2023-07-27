use std::fs::File;
use to_vec::ToVec;
use std::process::exit;
use std::io::{BufReader, BufRead, BufWriter, Write};

const ITERATE_MAX: i32 = 10000;
const DX: [i32; 8] = [1, 1, 1, 0, 0, -1, -1, -1];
const DY: [i32; 8] = [1, 0, -1, 1, -1, 1, 0, -1];
const INIT_FILENAME: &str = "initial.txt";

fn read_initial_data() -> (usize, usize, Vec<Vec<u8>>) {
    let fptr = match File::open(INIT_FILENAME) {
        Ok(fp) => fp,
        Err(_) => { println!("File doesn't exist"); exit(-1); }
    };

    let mut ret: Vec<Vec<u8>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let reader = BufReader::new(fptr);

    for (i, item) in reader.lines().enumerate() {
        let line = match item {
            Ok(v) => v,
            Err(_) => { println!("Error when open file."); exit(-1); }
        };

        let words = line.split_whitespace().map(|s| s.to_string()).to_vec();

        if i == 0 {
            assert_eq!(words.len(), 2);

            width = match words[0].parse::<usize>() {
                Ok(num) => num,
                Err(_) => 0,
            };

            height = match words[0].parse::<usize>() {
                Ok(num) => num,
                Err(_) => 0,
            };
        }
        else {
            assert_eq!(words.len(), width);
            ret.push(words.iter().map(|x| x.parse::<u8>().unwrap() * 255).collect());
        }
    };

    (width, height, ret)
}

fn run_game(width: &usize, height: &usize, input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut output:Vec<Vec<u8>> = vec![vec![0; *width]; *height];

    for i in 0..*height {
        for j in 0..*width {
            let mut cnt = 0;

            for k in 0..8 {
                let mut x: usize = (i as i32 + DX[k]) as usize;
                let mut y: usize = (j as i32 + DY[k]) as usize;

                if i as i32 + DX[k] < 0 {
                    x = *width - 1;
                }
                else if i as i32 + DX[k] >= *height as i32 {
                    x = 0;
                }

                if j as i32 + DY[k] < 0 {
                    y = *height - 1;
                }
                else if j as i32 + DY[k] >= *height as i32 {
                    y = 0;
                }

                if input[x][y] == 0 {
                    cnt += 0;
                }
                else {
                    cnt += 1;
                }

                // cnt += match input.get((i as i32 + DX[k]) as usize) {
                //     None => 0,
                //     Some(v) => {
                //         match v.get((j as i32 + DY[k]) as usize) {
                //             None => 0,
                //             Some(v) => {
                //                 if *v == 255 {
                //                     1
                //                 }
                //                 else {
                //                     0
                //                 }
                //             }
                //         }
                //     }
                // }
            }

            if input[i][j] == 0 {
                if cnt == 3 {
                    output[i][j] = 255;
                }
                else {
                    output[i][j] = 0;
                }
            } 
            else {
                if cnt == 3 || cnt == 2 {
                    output[i][j] = 255;
                }
                else {
                    output[i][j] = 0;
                }
            }
        }
    }

    output
}

fn save_data_as_ppm(width: &usize, height: &usize, input: &Vec<Vec<u8>>, iterate_count: &mut i32) {
    let file_name = format!("output/game_of_life_{}.ppm", *iterate_count);

    let fptr = match File::create(file_name) {
        Ok(fp) => fp,
        Err(_) => { println!("File open error."); return; }
    };

    let mut writer = BufWriter::new(fptr);

    let output = format!("{}\n{} {}\n{}\n", "P3", *width, *height, 255);
    match writer.write(output.as_bytes()) {
        Ok(_) => {},
        Err(_) => { println!("Error in write output"); exit(-1);}
    }

    for i in 0..*height {
        let mut output = String::new();

        for j in 0..*width {
            output.push_str(&format!("{} {} {} ", input[i][j], input[i][j], input[i][j]));
        }

        output.push('\n');

        match writer.write(output.as_bytes()) {
            Ok(_) => {},
            Err(_) => { println!("Error in write {}th output", i); exit(-1);}
        }
    }

    *iterate_count += 1;
}

fn main() {
    let (width, height, mut ret) = read_initial_data();
    let mut iterate_count = 0;

    for _ in 0..ITERATE_MAX {
        ret = run_game(&width, &height, ret);
        save_data_as_ppm(&width, &height, &ret, &mut iterate_count);
    }
}
