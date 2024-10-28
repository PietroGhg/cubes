use rand::Rng;
#[derive(PartialEq, PartialOrd, Debug)]
struct Point {
    z :f32,
    x :f32,
    y :f32,
    c :char,
    color : &'static str,
}

#[derive(Copy, Clone, Debug)]
struct Buf {
    c :char,
    color : &'static str,
    z :f32,
}


const S_H : usize = 80;
const S_W : usize = 150;
const MAX_L: f32 = 80.0; 
const MAX_H: f32 = 30.0; 
const MAX_Z: f32 = 20.0; 
const V : f32 = 2.0;
const A : f32 = 0.5;

// colors
const NEUTR : &'static str  = "\x1b[0m";
const _GREY : &'static str = "\x1b[90m";
const RED : &'static str = "\x1b[91m";
const GREEN : &'static str = "\x1b[92m";
const YELLOW : &'static str = "\x1b[93m";
const BLUE : &'static str = "\x1b[94m";
const PURPLE : &'static str = "\x1b[95m";
const CYAN : &'static str = "\x1b[96m";

type BufferT = [[Buf; S_W]; S_H];


fn rotate_point_x(p : &Point, a :f32) -> Point {
    Point {
        x : p.x,
        y: p.y * f32::cos(a) - p.z * f32::sin(a),
        z : p.y * f32::sin(a) + p.z * f32::cos(a),
        c : p.c,
        color : p.color,
    }
}

fn rotate_x(points :&Vec<Point>, a :f32) -> Vec<Point> {
    points.iter().map(|x| rotate_point_x(x,a)).collect()
}

fn rotate_point_y(p : &Point, a :f32) -> Point {
    Point {
        x : p.x * f32::cos(a) + p.z * f32::sin(a), 
        y: p.y,
        z : p.z * f32::cos(a) - p.x * f32::sin(a),
        c : p.c,
        color : p.color,
    }
}

fn rotate_y(points :&Vec<Point>, a :f32) -> Vec<Point> {
    points.iter().map(|x| rotate_point_y(x,a)).collect()
}

fn rotate_point_z(p : &Point, a :f32) -> Point {
    Point {
        x : p.x * f32::cos(a) - p.y * f32::sin(a), 
        y: p.x * f32::sin(a) + p.y * f32::cos(a) ,
        z : p.z, 
        c : p.c,
        color : p.color,
    }
}

fn rotate_z(points :&Vec<Point>, a :f32) -> Vec<Point> {
    points.iter().map(|x| rotate_point_z(x,a)).collect()
}

fn translate_point_x(p: &Point, x :f32, y :f32, z :f32) -> Point {
    Point {
        x : p.x + x,
        y : p.y + y,
        z : p.z + z,
        c : p.c,
        color : p.color,
    }
}

fn translate(points : &Vec<Point>, x : f32, y :f32, z :f32) -> Vec<Point> {
    points.iter().map(|p| translate_point_x(p,x,y,z)).collect()
}

struct Cube {
    x :f32,
    y :f32,
    z :f32,
    v_x :f32,
    v_y :f32,
    v_z :f32,
    a_x :f32,
    a_y :f32,
    a_z :f32,
    alpha_x :f32,
    alpha_y :f32,
    alpha_z :f32,
    points :Vec<Point>,
}

impl Cube {

fn new_colors(colors : [&'static str ; 6], l :usize, x :f32, y :f32, z :f32, v_x :f32, v_y : f32, v_z :f32, a_x :f32, a_y :f32, a_z :f32,  alpha_x :f32, alpha_y :f32, alpha_z :f32) -> Self {
    //let Faces :[char; 6] = ['.', '$', '^', '~', '#', '!'];

    let mut res : Vec<Point> = Vec::new();

    // front
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : i as f32, y : j as f32 , z : l as f32 / 2.0 , c : '.', color : colors[0]});
        }
    }
    // back
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : i as f32, y : j as f32 , z : -(l as f32) / 2.0 , c : '$', color : colors[1]});
        }
    }

    // right side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : l as f32 / 2.0 , y : j as f32 , z : i as f32, c : '^', color : colors[2]});
        }
    }
    // left side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : -(l as f32) / 2.0 , y : j as f32 , z : i as f32, c : '~', color : colors[3]});
        }
    }
    // top side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : j as f32 , y : l as f32 / 2.0 as f32 , z : i as f32, c : '#', color : colors[4]});
        }
    }
    // bottom side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : j as f32 , y : -(l as f32) / 2.0 as f32 , z : i as f32, c : '!', color : colors[5]});
        }
    }
    Cube {
        x,
        y,
        z,
        v_x,
        v_y,
        v_z,
        a_x,
        a_y,
        a_z,
        alpha_x,
        alpha_y,
        alpha_z,
        points: res,
    }

}

fn new(color : &'static str , l :usize, x :f32, y :f32, z :f32, v_x :f32, v_y : f32, v_z :f32, a_x :f32, a_y :f32, a_z :f32,  alpha_x :f32, alpha_y :f32, alpha_z :f32) -> Self {
    let colors = [color; 6];
    Cube::new_colors(colors, l, x, y, z, v_x, v_y, v_z, a_x, a_y, a_z, alpha_x, alpha_y, alpha_z)
}

fn tick(&mut self) {
        self.a_x = self.a_x + self.alpha_x;
        self.a_y = self.a_y + self.alpha_y;
        self.a_z = self.a_z + self.alpha_z;
        if (self.x + 3.0 * self.v_x).abs() > MAX_L { 
            self.v_x = -self.v_x;
        } else {
            self.x = self.x + self.v_x;
        }
        if (self.y + 3.0 * self.v_y).abs() > MAX_H { 
            self.v_y = -self.v_y;
        } else {
            self.y = self.y + self.v_y;
        }
        if (self.z + 3.0 * self.v_z).abs() > MAX_Z { 
            self.v_z = -self.v_z;
        } else {
            self.z = self.z + self.v_z;
        }
}

fn roto_transl(&self) -> Vec<Point> {
    // TODO use time
    let c_x = rotate_x(&self.points, self.a_x);
    let c_z = rotate_z(&c_x, self.a_z);
    let r = rotate_y(&c_z, self.a_y);
    let t = translate(&r, self.x, self.y, self.z);
    // TODO: since we don't have a view matrix we just translate 
    // the cubes way back in world coordinates
    let f = translate(&t, 0.0, 0.0, -50.0);
    return f;
}
}

fn display(points : &mut Vec<Point>, with_color : bool) {
    let mut buf : BufferT = [[ Buf {c: ' ', color: NEUTR, z: 0.0}; S_W]; S_H];

    // TODO: since we don't have a projection matrix we just divide x and y
    // by a factor * z to simulate perspective
    let mut projected_points : Vec<Point> = points.iter().map(|p| {
        let f = p.z * 0.05;
        Point {x:p.x/f, y:p.y/f, z:p.z, c:p.c, color : p.color}
    }).collect();
    projected_points.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());
    for p in projected_points {
        let x = (p.x + S_W as f32 / 2.0) as usize;
        let y = (-p.y  + S_H as f32 / 2.0) as usize;
        if x >= S_W || y >= S_H { continue; }
        let c = buf[y][x].c;
        let color = buf[y][x].color;
        let z = buf[y][x].z;
        buf[y][x].c = if c == ' ' { p.c } else { if p.z > z { p.c } else {c} };
        buf[y][x].color = if c == ' ' { p.color } else { if p.z > z { p.color } else { color } };
        buf[y][x].z = if c == ' ' { p.z } else { if p.z > z { p.z } else {z} };
    }
    let mut s :String = String::from("");

    for i in 0..S_H {
        for j in 0..S_W {
            let b = &buf[i as usize][j as usize];
            s += if with_color { b.color } else { "" };
            s.push(buf[i as usize][j as usize].c);
            s += if with_color { NEUTR } else { "" };
        }
        s.push('\n');
    }
    println!("{}", s);
}




fn main() {
    let mut cubes = Vec::new();
    let mut rng = rand::thread_rng();
    let args : Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cubes n");
        return;
    }
    let ncubes = str::parse::<usize>(&args[1]).expect("n arg must be an unsigned int");
    let colors = [RED, GREEN, BLUE, YELLOW, CYAN, PURPLE];
    for i in 0..ncubes {
        let v_x: f32 = rng.gen::<f32>() * V;
        let v_y: f32 = rng.gen::<f32>() * V;
        let v_z: f32 = rng.gen::<f32>() * V;
        let alpha_x: f32 = rng.gen::<f32>() * A;
        let alpha_y: f32 = rng.gen::<f32>() * A;
        let alpha_z: f32 = rng.gen::<f32>() * A;
        let idx : usize = rng.gen_range(0..6);
        let l : usize = rng.gen_range(5..10);
        cubes.push(Cube::new(colors[idx], l, i as f32, 0.0, 0.0, v_x, v_y, v_z, 0.0, 0.0, 0.0, alpha_x, alpha_y, alpha_z));
    }
    loop {
        print!("{}[2J", 27 as char);
        let mut pts = cubes.iter().flat_map(|c| c.roto_transl()).collect();
        display(&mut pts, true);
        for c in cubes.iter_mut() {
            c.tick();
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
