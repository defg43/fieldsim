use std::fmt;
extern crate line_drawing;
use line_drawing::Bresenham;
use line_drawing::Point;
use std::ops::Index;

/*
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
} */

#[derive(Debug)]
enum Cell {
	Air { potential: i32 }, 
	Metal { voltage: i32, pin:Option<u16> }, 
}

#[derive(Debug)]
enum ConductorShape {
	Line { p1: Point<i32>, p2: Point<i32> },
	Circle { origin: Point<i32>, radius: u32 },
	HlafCircle { origin: Point<i32>, radius: u32, angle: f64},
	Square { p1: Point<i32>, p2: Point<i32>, p3: Point<i32>, p4: Point<i32> }, 
}

#[derive(Debug)]
struct CellGrid {
	grid: [[Cell; 100]; 100], 
	conductor_count: i32,
	conductors: Vec<ConductorShape>,  
}

fn create_line(p1: Point<i32>, p2: Point<i32>) -> Option<Vec<Point<i32>>> {
	let mut result : Vec<Point<i32>> = Default::default(); 
	for (x, y) in Bresenham::new(p1, p2) {
		result.push((x, y));
	}
	return Some(result);
}

fn place_conductors_on_grid(conductor_to_place: Vec<ConductorShape>) -> Option<CellGrid> {
	let mut grid : Option<CellGrid> = Default::default();
	let mut buffer: Option<Vec<Point<i32>>> = Default::default();
	for conductor in conductor_to_place {
		match conductor {
			ConductorShape::Line { p1, p2 } => buffer = create_line(p1, p2), 
			ConductorShape::Circle { origin, radius } => todo!(), 
			ConductorShape::HlafCircle { origin, radius, angle } => todo!(),
			ConductorShape::Square { p1, p2, p3, p4 } => todo!(), 
		};
		
		if let Some(buffer) = buffer {
            if let Some(mut g) = grid.take() {
                for (x, y) in buffer.iter() {
                    g.grid[*x as usize][*y as usize] = Cell::Metal {
                        voltage: 0,
                        pin: None,
                    };
                }
                grid = Some(g);
            }
        }
    }
	return grid;
}

fn main() {
    println!("Hello, world!");
}
