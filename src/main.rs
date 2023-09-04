struct Point {
    x: f64,
    y: f64,
}

enum Cell {
	Air { potential: i32 }, 
	Metal { voltage: i32, pin:u16 }, 
}

enum ConductorShape {
	Line { p1: Point, p2: Point },
	Circle { origin: Point, radius: u32 },
	HlafCircle { origin: Point, radius: u32, angle: f64},
	Square { p1: Point, p2: Point, p3: Point, p4: Point }, 
}

struct CellGrid {
	grid: [[Cell; 100]; 100], 
	conductor_count: i32,
	conductors: [ConductorShape],  
}

fn place_conductors_on_grid(conductor_to_place: [ConductorShape; 10] ) -> Option<CellGrid> {
	
}

fn main() {
    println!("Hello, world!");
}
