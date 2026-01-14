use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

/*--------------------------------------
  定数
--------------------------------------*/
const HEIGHT: usize = 40;
const WIDTH: usize = 70;

/*--------------------------------------
  main
--------------------------------------*/
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("usage: {} [filename for init]", args[0]);
        return Ok(());
    }

    let fp = if args.len() == 2 {
        Some(BufReader::new(File::open(&args[1])?))
    } else {
        None
    };

    let mut cell = vec![vec![0i32; WIDTH]; HEIGHT];

    my_init_cells(HEIGHT, WIDTH, &mut cell, fp);

    let mut generation: i32 = 0;
    let mut out = io::stdout();

    my_print_cells(&mut out, generation, HEIGHT, WIDTH, &cell);
    sleep(Duration::from_secs(1));
    write!(out, "\x1b[{}A", HEIGHT + 3)?;
    out.flush()?;

    loop {
        generation += 1;
        my_update_cells(HEIGHT, WIDTH, &mut cell);
        my_print_cells(&mut out, generation, HEIGHT, WIDTH, &cell);
        sleep(Duration::from_secs(1));
        write!(out, "\x1b[{}A", HEIGHT + 3)?;
        out.flush()?;
    }
}

/*--------------------------------------
  周囲セル数カウント
--------------------------------------*/
fn my_count_adjacent_cells(
    height: usize,
    width: usize,
    cell: &Vec<Vec<i32>>,
    y: isize,
    x: isize,
) -> i32 {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let ny = y + dy;
            let nx = x + dx;

            if ny < 0 || ny >= height as isize {
                continue;
            }
            if nx < 0 || nx >= width as isize {
                continue;
            }

            count += cell[ny as usize][nx as usize];
        }
    }
    count
}

/*--------------------------------------
  初期化
--------------------------------------*/
fn my_init_cells(
    height: usize,
    width: usize,
    cell: &mut Vec<Vec<i32>>,
    fp: Option<BufReader<File>>,
) {
    for y in 0..height {
        for x in 0..width {
            cell[y][x] = 0;
        }
    }

    // default
    if fp.is_none() {
        let points = [(20, 30), (20, 32), (22, 30), (22, 31), (23, 31)];
        for (y, x) in points {
            cell[y][x] = 1;
        }
        return;
    }

    let mut px: isize = 0;
    let mut py: isize = 0;

    let reader = fp.unwrap();

    for line in reader.lines().flatten() {
        // 原点指定
        if let Some(rest) = line.strip_prefix("#P") {
            let nums: Vec<_> = rest.split_whitespace().collect();
            if nums.len() == 2 {
                if let (Ok(x), Ok(y)) = (nums[0].parse(), nums[1].parse()) {
                    px = x;
                    py = y;
                }
            }
            continue;
        }

        // コメント行は無視
        if line.starts_with('#') {
            continue;
        }

        // セル座標
        let nums: Vec<_> = line.split_whitespace().collect();
        if nums.len() == 2 {
            if let (Ok(x), Ok(y)) = (nums[0].parse::<isize>(), nums[1].parse::<isize>()) {
                let xx = x + px;
                let yy = y + py;

                if yy >= 0 && yy < height as isize &&
                   xx >= 0 && xx < width as isize {
                    cell[yy as usize][xx as usize] = 1;
                }
            }
        }
    }
}

/*--------------------------------------
  表示
--------------------------------------*/
fn my_print_cells<W: Write>(
    fp: &mut W,
    generation: i32,
    height: usize,
    width: usize,
    cell: &Vec<Vec<i32>>,
) {
    writeln!(fp, "generateion = {}", generation).unwrap();

    write!(fp, "+").unwrap();
    for _ in 0..width {
        write!(fp, "-").unwrap();
    }
    writeln!(fp, "+").unwrap();

    for y in 0..height {
        write!(fp, "|").unwrap();
        for x in 0..width {
            if cell[y][x] != 0 {
                write!(fp, "\x1b[31m#\x1b[0m").unwrap();
            } else {
                write!(fp, " ").unwrap();
            }
        }
        writeln!(fp, "|").unwrap();
    }

    write!(fp, "+").unwrap();
    for _ in 0..width {
        write!(fp, "-").unwrap();
    }
    writeln!(fp, "+").unwrap();
}

/*--------------------------------------
  世代更新
--------------------------------------*/
fn my_update_cells(height: usize, width: usize, cell: &mut Vec<Vec<i32>>) {
    let mut next = vec![vec![0i32; width]; height];

    for y in 0..height {
        for x in 0..width {
            let n = my_count_adjacent_cells(height, width, cell, y as isize, x as isize);
            if cell[y][x] != 0 {
                next[y][x] = if n == 2 || n == 3 { 1 } else { 0 };
            } else {
                next[y][x] = if n == 3 { 1 } else { 0 };
            }
        }
    }

    *cell = next;
}

