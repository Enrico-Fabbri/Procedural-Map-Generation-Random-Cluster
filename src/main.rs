use rand::Rng;
use std::fs::File;
use std::io::Write;
use text_io::read;

fn main() {
    menu_management();
}

fn menu_management() {
    loop {
        println!("Random-Cluster Procedural Generation Project\n\t- START: write \"s\" to start\n\t- EXIT: write \"e\" to exit the program");

        let response: String = read!();

        if response == "s" {
            // Map data
            let mut map: Vec<Vec<u8>> = Vec::new();

            println!("Write map width and height:");
            let width: u32 = read!();
            let height: u32 = read!();

            // Generate map
            gen_map(width, height, &mut map);

            // Miner data
            let mut steps = Vec::new();

            // Miner Steps Data
            println!("Write minimum and maximum number of possible steps:");
            let min_s: u32 = read!();
            let max_s: u32 = read!();

            // Generate steps
            gen_steps(min_s, max_s, &mut steps);

            // Miner
            let mut miner = (width / 2, height / 2);

            // Update the map
            update_map(&steps, &mut map, &mut miner, width, height);

            // Get file data
            println!("Write the file path, file name and extension");
            let file_path: String = read!();

            // Generate directory and file if not existing
            let path = std::path::Path::new(&file_path);
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            let mut file = File::create(path).unwrap();

            // Save map
            save_map(width, height, &map, &mut file);
        }
        if response == "e" {
            break;
        }
    }
}

/// Generate map
pub fn gen_map(width: u32, height: u32, map: &mut Vec<Vec<u8>>) {
    for _ in 0..width {
        map.push(vec![0; height as usize]);
    }
}

/// Generate random steps
pub fn gen_steps(min_s: u32, max_s: u32, steps: &mut Vec<i32>) {
    let mut rand = rand::thread_rng();

    if max_s > min_s {
        for _ in min_s..max_s {
            steps.push(rand.gen_range(0..4));
        }
    } else {
        for _ in max_s..min_s {
            steps.push(rand.gen_range(0..4));
        }
    }
}

/// Update the map
pub fn update_map(
    steps: &[i32],
    map: &mut [Vec<u8>],
    miner: &mut (u32, u32),
    width: u32,
    height: u32,
) {
    for v in steps {
        (map[miner.0 as usize])[miner.1 as usize] = 1;

        if v == &0 {
            // Left
            if miner.0 == 0 && miner.1 == 0 {
                miner.0 = width - 1;
                miner.1 = height - 1;
            } else if miner.0 == 0 {
                miner.0 = width - 1;
                miner.1 -= 1;
            } else {
                miner.0 -= 1;
            }
        } else if v == &1 {
            // Up
            if miner.1 == 0 {
                miner.1 = height - 1;
            } else {
                miner.1 -= 1;
            }
        } else if v == &2 {
            // Right
            if miner.0 == width - 1 && miner.1 == height - 1 {
                miner.0 = 0;
                miner.1 = 0;
            } else if miner.0 == width - 1 {
                miner.0 = 0;
                miner.1 += 1;
            } else {
                miner.0 += 1;
            }
        } else if v == &3 {
            // Down
            if miner.1 == height - 1 {
                miner.1 = 0;
            } else {
                miner.1 += 1;
            }
        }
    }
}

/// Save the map
pub fn save_map(width: u32, height: u32, map: &[Vec<u8>], file: &mut File) {
    for x in 0..width {
        let vx = map.get(x as usize).unwrap();
        for y in 0..height {
            if vx.get(y as usize).unwrap() == &0 {
                //print!("#");
                if let Err(e) = write!(file, "#") {
                    print!("{:?}", e);
                }
            } else {
                //print!(".");
                if let Err(e) = write!(file, ".") {
                    print!("{:?}", e);
                }
            }
        }
        if let Err(e) = writeln!(file) {
            print!("{:?}", e);
        }
    }
}
