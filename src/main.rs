use std::fmt;
use std::f64;
use std::collections::HashMap;
extern crate line_drawing;
use line_drawing::Bresenham;
use line_drawing::BresenhamCircle;
use line_drawing::Point;
use std::ops::Index;
use std::sync::atomic::{AtomicU16, Ordering};
use colored::*;

#[derive(Debug, Clone, Copy)]
enum Cell {
	Air { potential: i32 }, 
	Metal { voltage: i32, pin:Option<u16> }, 
}

#[derive(Debug, Copy, Clone)]
enum ConductorShape {
	Line { p1: Point<i32>, p2: Point<i32> },
	Circle { origin: Point<i32>, radius: i32 },
	HlafCircle { origin: Point<i32>, radius: i32, angle: f64},
	Square { p1: Point<i32>, p2: Point<i32>, p3: Point<i32>, p4: Point<i32> }, 
}

#[derive(Debug)]
struct CellGrid {
	grid: [[Cell; 100]; 100], 
	conductor_count: i32,
	conductors: Vec<Point<i32>>,  
}

impl Clone for CellGrid {
    fn clone(&self) -> Self {
        CellGrid {
            grid: self.grid.clone(),
            conductor_count: self.conductor_count,
            conductors: self.conductors.clone(),
        }
    }
}

static NEXT_PIN_INDEX: AtomicU16 = AtomicU16::new(0);

fn generate_pin_index() -> u16 {
    NEXT_PIN_INDEX.fetch_add(1, Ordering::Relaxed)
}

fn create_line(p1: Point<i32>, p2: Point<i32>) -> Option<Vec<Point<i32>>> {
	let mut result : Vec<Point<i32>> = Default::default(); 
	for (x, y) in Bresenham::new(p1, p2) {
		result.push((x, y));
	}
	return Some(result);
}

fn create_circle(origin: Point<i32>, radius: i32) -> Option<Vec<Point<i32>>> {
	let mut result : Vec<Point<i32>> = Default::default(); 
	for (x, y) in BresenhamCircle::new(origin.0, origin.1, radius) {
		result.push((x, y));
	}
	return Some(result);
}

fn apply_voltage_from_pins(input: &mut CellGrid, voltages: HashMap<u16, i32>) {
    for x in 0..input.grid.len() {
        for y in 0..input.grid[x].len() {
            match &mut input.grid[x][y] {
                Cell::Metal { voltage: v, pin: Some(pin) } => {
                    if let Some(&new_voltage) = voltages.get(&pin) {
                        *v = new_voltage;
                    }
                },
                Cell::Metal { voltage: v, pin: None } => {
                    *v = 0;
                },
                Cell::Air { potential: _p } => {},
            }
        }
    }
}

fn calculate_electric_potential(input: &mut CellGrid) {
    let mut new_grid = input.grid.clone(); // Create a new grid to store updated potentials

    for x in 0..input.grid.len() {
        for y in 0..input.grid[x].len() {
            match &input.grid[x][y] {
                Cell::Air { potential } => {
                    let mut new_potential = 0;
                    for i in 0..input.grid.len() {
                        for j in 0..input.grid[i].len() {
                            if let Cell::Metal { voltage, .. } = &input.grid[i][j] {
                                let distance = ((x as i32 - i as i32).pow(2) + (y as i32 - j as i32).pow(2)) as f64;
                                if distance != 0.0 {
                                    new_potential += voltage / distance.sqrt() as i32;
                                }
                            }
                        }
                    }
                    // Update the potential in the new grid
                    new_grid[x][y] = Cell::Air {
                        potential: new_potential as i32,
                    };
                }
                _ => {} // Skip metal cells
            }
        }
    }

    // Copy the new grid back to the original grid
    input.grid = new_grid;
}

fn place_conductors_on_grid(conductor_to_place: Vec<ConductorShape>) -> Option<CellGrid> {
    let mut grid = Some(CellGrid {
        grid: [[Cell::Air { potential: 0 }; 100]; 100],
        conductor_count: 0,
        conductors: Vec::new(),
    });

    for conductor in conductor_to_place {
        let mut buffer: Option<Vec<Point<i32>>> = None;

        match conductor {
            ConductorShape::Line { p1, p2 } => {
                buffer = create_line(p1, p2);
            }
            ConductorShape::Circle { origin, radius } => {
                buffer = create_circle(origin, radius);
            }
            ConductorShape::HlafCircle { origin, radius, angle } => todo!(),
            ConductorShape::Square { p1, p2, p3, p4 } => todo!(),
        };

        if let Some(mut g) = grid.take() {
            if let Some(buffer) = buffer {
				let pinid = generate_pin_index();
                for (x, y) in buffer.iter() {
                    g.grid[*x as usize][*y as usize] = Cell::Metal {
                        voltage: 0,
                        pin: Some(pinid),
                    };
                }
                g.conductor_count += 1;
                g.conductors.extend(buffer);
                grid = Some(g);
            }
        }
    }

    grid
}


fn mainold() {
	let foo: ConductorShape = ConductorShape::Line { p1: (0,0), p2: (99,99) };
	let bar: ConductorShape = ConductorShape::Circle { origin: (30, 10), radius: 5 };
	let mut gr: Option<CellGrid> = place_conductors_on_grid(vec![foo, bar]);
	let mut voltages: HashMap<u16, i32> = HashMap::new();
    voltages.insert(0, 10);
    voltages.insert(1, 5);
	if let Some(ref mut grid) = gr {
		apply_voltage_from_pins(grid, voltages);
		calculate_electric_potential(grid);
		print!("{:#?}", grid.grid[1][2]);
	}
	let mut pin_id_string = String::new();
	if let Some(cell_grid) = gr {
        for (x, row) in cell_grid.grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
				match cell {
					Cell::Metal { voltage: _v, pin: Some(pinid) } => {
						print!("█");
						pin_id_string.push_str(&pinid.to_string());
					},
					Cell::Metal { voltage: _, pin: None } => {
						print!("█");
						pin_id_string.push_str("x");
					}, 
					Cell::Air { potential: _p } => {
						print!(" ");
						pin_id_string.push_str(" ");
					},
				};
            }
			print!("\n");
			pin_id_string.push_str("\n");
        }
		print!("{}", pin_id_string);
    } else {
        println!("Failed to create the cell grid.");
    }
	//dbg!(bar);
}


fn main() {
    let mut gr: Option<CellGrid> = 
	place_conductors_on_grid(vec![
		ConductorShape::Line {
			p1: (2, 2), p2: (2, 97) 
		}, 
		ConductorShape::Line {
			p1: (2, 97), p2: (40, 97) 
		}, 
		ConductorShape::Line {
			p1: (40, 2), p2: (40, 97) 
		}, 
		ConductorShape::Line {
			p1: (2, 2), p2: (40, 2) 
		}, 
		ConductorShape::Line {
			p1: (8, 22), p2: (25, 22) 
		}, 
		ConductorShape::Line {
			p1: (32, 35), p2: (32, 80) 
		}, 
	]);
    let mut voltages: HashMap<u16, i32> = HashMap::new();
    voltages.insert(0, -80);
    voltages.insert(1, -80);
	voltages.insert(2, -80);
	voltages.insert(3, -80);
	voltages.insert(4, 80);
	voltages.insert(5, 80);
    if let Some(ref mut grid) = gr {
        apply_voltage_from_pins(grid, voltages);
        calculate_electric_potential(grid);

        // Find the minimum and maximum potential in air cells
        let mut min_potential = i32::MAX;
        let mut max_potential = i32::MIN;

        for x in 0..grid.grid.len() {
            for y in 0..grid.grid[x].len() {
                if let Cell::Air { potential } = grid.grid[x][y] {
                    if potential < min_potential {
                        min_potential = potential;
                    }
                    if potential > max_potential {
                        max_potential = potential;
                    }
                }
            }
        }

        // Calculate the step height for the color gradient
        let range= (max_potential - min_potential) as f64;
	    for x in 0..grid.grid.len() {
            for y in 0..grid.grid[x].len() {
                match grid.grid[x][y] {
					Cell::Metal { voltage: _voltage, pin: _pin } => { print!("{}", "█".black().on_white());},
					Cell::Air { potential: potential } => { 
						print!("{}", " "
						.on_truecolor((((potential - min_potential) as f64 / range) * 255.00) as u8, 
						( (1.0 - ((potential - min_potential) as f64 / range)) * 255.00) as u8, 125)); },
                }
            }
            println!();
        }
    } else {
        println!("Failed to create the cell grid.");
    }
}
