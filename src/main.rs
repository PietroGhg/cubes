#[derive(PartialEq, PartialOrd, Debug)]
struct Point {
    z :f32,
    x :f32,
    y :f32,
    c :char,
}

#[derive(Copy, Clone, Debug)]
struct Buf {
    c :char,
    z :f32,
}


const S_H : usize = 80;
const S_W : usize = 120;
const MAX_L: f32 = 45.0; 
const V : f32 = 0.5;
type BufferT = [[Buf; S_W]; S_H];


fn rotate_point_x(p : &Point, a :f32) -> Point {
    Point {
        x : p.x,
        y: p.y * f32::cos(a) - p.z * f32::sin(a),
        z : p.y * f32::sin(a) + p.z * f32::cos(a),
        c : p.c,
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
    }
}

fn translate(points : &Vec<Point>, x : f32, y :f32, z :f32) -> Vec<Point> {
    points.iter().map(|p| translate_point_x(p,x,y,z)).collect()
}

struct Cube {
    x :f32,
    y :f32,
    z :f32,
    v :f32,
    a_x :f32,
    a_y :f32,
    a_z :f32,
    alpha_x :f32,
    alpha_y :f32,
    alpha_z :f32,
    points :Vec<Point>,
}

impl Cube {

fn new(l :usize, x :f32, y :f32, z :f32, v :f32, a_x :f32, a_y :f32, a_z :f32,  alpha_x :f32, alpha_y :f32, alpha_z :f32) -> Self {
    //let Faces :[char; 6] = ['.', '$', '^', '~', '#', '!'];

    let mut res : Vec<Point> = Vec::new();

    // front
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : i as f32, y : j as f32 , z : l as f32 / 2.0 , c : '.'});
        }
    }
    // back
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : i as f32, y : j as f32 , z : -(l as f32) / 2.0 , c : '$'});
        }
    }

    // right side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : l as f32 / 2.0 , y : j as f32 , z : i as f32, c : '^'});
        }
    }
    // left side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : -(l as f32) / 2.0 , y : j as f32 , z : i as f32, c : '~'});
        }
    }
    // top side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : j as f32 , y : l as f32 / 2.0 as f32 , z : i as f32, c : '#'});
        }
    }
    // bottom side
    for i in -(l as i32)/2..(l as i32 /2) {
        for j in -(l as i32)/2..(l as i32 /2) {
            res.push(Point{ x : j as f32 , y : -(l as f32) / 2.0 as f32 , z : i as f32, c : '!'});
        }
    }
    Cube {
        x,
        y,
        z,
        v,
        a_x,
        a_y,
        a_z,
        alpha_x,
        alpha_y,
        alpha_z,
        points: res,
    }

}

fn tick(&mut self) {
        self.a_x = self.a_x + self.alpha_x;
        self.a_y = self.a_y + self.alpha_y;
        self.a_z = self.a_z + self.alpha_z;
        if (self.x + 3.0 * self.v).abs() > MAX_L { 
            self.v = -self.v;
        } else {
            self.x = self.x + self.v;
        }
}

fn roto_transl(&self) -> Vec<Point> {
    // TODO use time
    let c_x = rotate_x(&self.points, self.a_x);
    let c_z = rotate_z(&c_x, self.a_z);
    let r = rotate_y(&c_z, self.a_y);
    let t = translate(&r, self.x, self.y, self.z);
    return t;
}
}

fn display(points : &mut Vec<Point>) {
    let mut buf : BufferT = [[ Buf {c: ' ', z: 0.0}; S_W]; S_H];

    points.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());
    for p in points {
        let x = (p.x + S_W as f32 / 2.0) as usize;
        let y = (-p.y  + S_H as f32 / 2.0) as usize;
        let c = buf[y][x].c;
        let z = buf[y][x].z;
        buf[y][x].c = if c == ' ' { p.c } else { if p.z > z { p.c } else {c} };
        buf[y][x].z = if c == ' ' { p.z } else { if p.z > z { p.z } else {z} };
    }
    let mut s :String = String::from("");

    for i in 0..S_H {
        for j in 0..S_W {
            s.push(buf[i as usize][j as usize].c);
            s.push(' ');
        }
        s.push('\n');
    }
    println!("{}", s);
}




fn main() {
    let mut cubes = Vec::new();
    let c1 = Cube::new(10, 0.0, 0.0, 0.0, V+0.3, 0.0, 0.0, 2.8, 0.2, 0.0, -0.1);
    let c2= Cube::new(8, 10.0, -15.0, 0.0, V+0.4, 0.0, 0.0, -2.8, 0.0, 0.1, 0.1);
    let c3= Cube::new(9, 10.0, -30.0, 0.0, V+0.2, 0.0, 0.0, -2.8, 0.1, 0.1, 0.0);
    cubes.push(c1);
    cubes.push(c2);
    cubes.push(c3);
    loop {
        print!("{}[2J", 27 as char);
        let mut pts = cubes.iter().flat_map(|c| c.roto_transl()).collect();
        display(&mut pts);
        for c in cubes.iter_mut() {
            c.tick();
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
