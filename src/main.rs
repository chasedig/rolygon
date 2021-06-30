use std::*;
use colored::*;
use std::io::Write;

fn main() {
    let mut points: std::vec::Vec<Point> = std::vec::Vec::new();

    if std::env::consts::OS == "windows" { // windows only issue (terminal colors breaks on windows)
        control::set_virtual_terminal(true).expect("set virtual terminal failed");
    }

    println!("{}", "Type \"help\" to get a list of commands.".bold().white());

    loop {
        print!("{}", "Type a command (help) or enter the coordinates of a new point (x, y): ".bold().green());
        io::stdout().flush().unwrap();
        let _input = &mut String::new();
        std::io::stdin().read_line(_input).expect("reading input errored");
        println!();

        let input: String = _input.replace(" ", "").trim().to_owned().to_lowercase();

        let pointres = evaluate_coordinates(&input);


        if input == "help" {
            println!("{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "HELP MENU: ".bold().yellow(),
            "help: get a list of commands".yellow(),
            "perimeter: get the perimeter".yellow(),
            "undo: remove the last point added to the polygon".yellow(),
            "list: get a list of points".yellow(),
            "reset: clears the polygon of its points".yellow(),
            "exit: close rolygon".yellow()
            );
        }
        else if input == "perimeter" {
            let mut perimeter: f32 = 0.0;
            let mut last_point: &Point;
            for i in 1..points.len() {
                let point = &points[i];
                last_point = &points[i-1];
                perimeter += get_distance(last_point, point);
            }
            println!("{}", format!("The perimeter of the polygon is {}.", perimeter).bold().blue());
        }
        else if input == "undo" {
            if points.len() > 0
            {
                let p_index = points.len()-1;
                println!("{}", format!("The previous point #{} ({}) was undone.", points.len(), points[p_index]).bold().green());
                points.remove(p_index);
            }
            else
            {
                println!("{}", "There is nothing to undo.".bold().red());
            }
        }
        else if input == "list" {
            if points.len() > 0 {
                for (i, point) in points.iter().enumerate() {
                    println!("{}", format!("Point {}: {}", i+1, point).bold().white());
                }
            }
            else {
                println!("{}", "No points have been added yet.".bold().white());
            }
        }
        else if input == "reset" {
            println!("{}", format!("The polygon was reset, and {} points were cleared.", points.len()).red());
            points.clear();
        }
        else if input == "exit" {
            for _n in 1..100 {
                println!();
            }
            println!("{}", "Goodbye!".bold().bright_red());
            thread::sleep(std::time::Duration::from_millis(1000));
            std::process::exit(0);
        }
        else if pointres.is_ok() {
            let point = pointres.unwrap();
            println!("{}", format!("Coordinate {} added!", point.to_string()).bold().white());
            points.push(point);
        }
        else {
            println!("{}", "Your input was invalid. Type \"help\" to see what you can do.".red());
        }

    }
}

fn get_distance(p1: &Point, p2: &Point) -> f32 {
    ( (p2.y-p1.y).powf(2.0) + (p2.x-p1.x).powf(2.0) ).sqrt()
}

fn evaluate_coordinates(coord_string: &String) -> Result<Point, String> {
    let input: String = coord_string.replace(" ", "").trim().to_owned();
    let coordinates: Vec<_> = input.split(",").collect();
    if coordinates.len() == 2 {
        let xres = coordinates[0].to_string().parse();
        let yres = coordinates[1].to_string().parse();
        if xres.is_ok() && yres.is_ok() {
            let x: f32 = xres.unwrap();
            let y: f32 = yres.unwrap();
            let point: Point = Point {x, y};
            Ok(point)
        } else {
            Err("values were not numeric".to_string())
        }
    } else {
        Err(format!("expected 2 coordinates, received {}", coordinates.len()))
    }
}

struct Point
{
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}