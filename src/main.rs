use rand::Rng;
use std::time::Instant;

#[derive(PartialEq, PartialOrd, Debug)]
struct Vec3 {
    z: f32,
    x: f32,
    y: f32,
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Point {
    pos: Vec3,
    c: char,
    color: &'static str,
}

#[derive(Copy, Clone, Debug)]
struct Buf {
    c: char,
    color: &'static str,
    z: f32,
}

const S_H: usize = 80;
const S_W: usize = 200;
const MAX_L: f32 = S_W as f32 / 2.0;
const MAX_H: f32 = S_H as f32 / 2.0;
const MAX_Z: f32 = 20.0;
const V: f32 = 0.04; // units of space / milliseconds
const A: f32 = 0.02; // rad / milliseconds

// colors
const NEUTR: &str = "\x1b[0m";
const _GREY: &str = "\x1b[90m";
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const PURPLE: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";

type BufferT = [[Buf; S_W]; S_H];

fn dist(p1: &Vec3, p2: &Vec3) -> f32 {
    f32::sqrt(f32::powi(p1.x - p2.x, 2) + f32::powi(p1.y - p2.y, 2) + f32::powi(p1.z - p2.z, 2))
}

fn rotate_point_x(p: &Point, a: f32) -> Point {
    Point {
        pos: Vec3 {
            x: p.pos.x,
            y: p.pos.y * f32::cos(a) - p.pos.z * f32::sin(a),
            z: p.pos.y * f32::sin(a) + p.pos.z * f32::cos(a),
        },
        c: p.c,
        color: p.color,
    }
}

fn rotate_x(points: &[Point], a: f32) -> Vec<Point> {
    points.iter().map(|x| rotate_point_x(x, a)).collect()
}

fn rotate_point_y(p: &Point, a: f32) -> Point {
    Point {
        pos: Vec3 {
            x: p.pos.x * f32::cos(a) + p.pos.z * f32::sin(a),
            y: p.pos.y,
            z: p.pos.z * f32::cos(a) - p.pos.x * f32::sin(a),
        },
        c: p.c,
        color: p.color,
    }
}

fn rotate_y(points: &[Point], a: f32) -> Vec<Point> {
    points.iter().map(|x| rotate_point_y(x, a)).collect()
}

fn rotate_point_z(p: &Point, a: f32) -> Point {
    Point {
        pos: Vec3 {
            x: p.pos.x * f32::cos(a) - p.pos.y * f32::sin(a),
            y: p.pos.x * f32::sin(a) + p.pos.y * f32::cos(a),
            z: p.pos.z,
        },
        c: p.c,
        color: p.color,
    }
}

fn rotate_z(points: &[Point], a: f32) -> Vec<Point> {
    points.iter().map(|x| rotate_point_z(x, a)).collect()
}

fn translate_point_x(p: &Point, x: f32, y: f32, z: f32) -> Point {
    Point {
        pos: Vec3 {
            x: p.pos.x + x,
            y: p.pos.y + y,
            z: p.pos.z + z,
        },
        c: p.c,
        color: p.color,
    }
}

fn translate(points: &[Point], x: f32, y: f32, z: f32) -> Vec<Point> {
    points
        .iter()
        .map(|p| translate_point_x(p, x, y, z))
        .collect()
}

struct Cube {
    l: f32,
    pos: Vec3,
    v: Vec3,
    a: Vec3,
    alpha: Vec3,
    time: Instant,
    points: Vec<Point>,
}

impl Cube {
    fn new_colors(
        colors: [&'static str; 6],
        l: usize,
        pos: Vec3,
        v: Vec3,
        a: Vec3,
        alpha: Vec3,
    ) -> Self {
        //let Faces :[char; 6] = ['.', '$', '^', '~', '#', '!'];

        let mut res: Vec<Point> = Vec::new();

        // front
        for i in -(l as i32) / 2..(l as i32 / 2) {
            for j in -(l as i32) / 2..(l as i32 / 2) {
                res.push(Point {
                    pos: Vec3 {
                        x: i as f32,
                        y: j as f32,
                        z: l as f32 / 2.0,
                    },
                    c: '.',
                    color: colors[0],
                });
            }
        }
        // back
        for i in -(l as i32) / 2..(l as i32 / 2) {
            for j in -(l as i32) / 2..(l as i32 / 2) {
                res.push(Point {
                    pos: Vec3 {
                        x: i as f32,
                        y: j as f32,
                        z: -(l as f32) / 2.0,
                    },
                    c: '$',
                    color: colors[1],
                });
            }
        }

        // right side
        for i in -(l as i32) / 2..(l as i32 / 2) {
            for j in -(l as i32) / 2..(l as i32 / 2) {
                res.push(Point {
                    pos: Vec3 {
                        x: l as f32 / 2.0,
                        y: j as f32,
                        z: i as f32,
                    },
                    c: '^',
                    color: colors[2],
                });
            }
        }
        // left side
        for i in -(l as i32) / 2..(l as i32 / 2) {
            for j in -(l as i32) / 2..(l as i32 / 2) {
                res.push(Point {
                    pos: Vec3 {
                        x: -(l as f32) / 2.0,
                        y: j as f32,
                        z: i as f32,
                    },
                    c: '~',
                    color: colors[3],
                });
            }
        }
        // top side
        for i in -(l as i32) / 2..(l as i32 / 2) {
            for j in -(l as i32) / 2..(l as i32 / 2) {
                res.push(Point {
                    pos: Vec3 {
                        x: j as f32,
                        y: l as f32 / 2.0,
                        z: i as f32,
                    },
                    c: '#',
                    color: colors[4],
                });
            }
        }
        // bottom side
        for i in -(l as i32) / 2..(l as i32 / 2) {
            for j in -(l as i32) / 2..(l as i32 / 2) {
                res.push(Point {
                    pos: Vec3 {
                        x: j as f32,
                        y: -(l as f32) / 2.0,
                        z: i as f32,
                    },
                    c: '!',
                    color: colors[5],
                });
            }
        }
        Cube {
            l: l as f32,
            pos,
            v,
            a,
            alpha,
            time: Instant::now(),
            points: res,
        }
    }

    fn new(color: &'static str, l: usize, pos: Vec3, v: Vec3, a: Vec3, alpha: Vec3) -> Self {
        let colors = [color; 6];
        Cube::new_colors(colors, l, pos, v, a, alpha)
    }

    fn will_collide(&self, others: &[Cube]) -> bool {
        let mut will_collide = false;
        // TODO we should do the computation in tick(), or have them share the same instant
        let now = Instant::now();
        let delta_t = now.duration_since(self.time).as_millis() as f32;
        for other in others.iter() {
            if self.pos == other.pos {
                continue;
            }
            let delta = dist(
                &Vec3 {
                    x: self.pos.x + self.v.x * delta_t,
                    y: self.pos.y + self.v.y * delta_t,
                    z: self.pos.z + self.v.z * delta_t,
                },
                &Vec3 {
                    x: other.pos.x + other.v.x * delta_t,
                    y: other.pos.y + other.v.y * delta_t,
                    z: other.pos.z + other.v.z * delta_t,
                },
            );

            if delta < self.l * f32::sqrt(2.0) / 2.0 + other.l * f32::sqrt(2.0) / 2.0 {
                will_collide = true;
            }
        }
        will_collide
    }

    fn tick(&mut self, will_collide: bool) {
        let now = Instant::now();
        let delta_t = now.duration_since(self.time).as_millis() as f32;
        self.time = now;
        self.a.x += self.alpha.x * delta_t;
        self.a.y += self.alpha.y * delta_t;
        self.a.z += self.alpha.z * delta_t;
        if (self.pos.x + self.v.x * delta_t).abs() > MAX_L || will_collide {
            self.v.x = -self.v.x;
        } else {
            self.pos.x += self.v.x * delta_t;
        }
        if (self.pos.y + self.v.y * delta_t).abs() > MAX_H || will_collide {
            self.v.y = -self.v.y;
        } else {
            self.pos.y += self.v.y * delta_t;
        }
        if (self.pos.z + self.v.z * delta_t).abs() > MAX_Z || will_collide {
            self.v.z = -self.v.z;
        } else {
            self.pos.z += self.v.z * delta_t;
        }
    }

    fn roto_transl(&self) -> Vec<Point> {
        let c_x = rotate_x(&self.points, self.a.x);
        let c_z = rotate_z(&c_x, self.a.z);
        let r = rotate_y(&c_z, self.a.y);
        let t = translate(&r, self.pos.x, self.pos.y, self.pos.z);
        // TODO: since we don't have a view matrix we just translate
        // the cubes way back in world coordinates
        translate(&t, 0.0, 0.0, -50.0)
    }
}

fn display(points: &mut [Point], with_color: bool) {
    let mut buf: BufferT = [[Buf {
        c: ' ',
        color: NEUTR,
        z: 0.0,
    }; S_W]; S_H];

    // TODO: since we don't have a projection matrix we just divide x and y
    // by a factor * z to simulate perspective
    let mut projected_points: Vec<Point> = points
        .iter()
        .map(|p| {
            let f = p.pos.z * 0.05;
            Point {
                pos: Vec3 {
                    x: p.pos.x / f,
                    y: p.pos.y / f,
                    z: p.pos.z,
                },
                c: p.c,
                color: p.color,
            }
        })
        .collect();
    projected_points.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());
    for p in projected_points {
        let x = (p.pos.x + S_W as f32 / 2.0) as usize;
        let y = (-p.pos.y + S_H as f32 / 2.0) as usize;
        if x >= S_W || y >= S_H {
            continue;
        }
        let c = buf[y][x].c;
        let color = buf[y][x].color;
        let z = buf[y][x].z;
        buf[y][x].c = if c == ' ' || p.pos.z > z { p.c } else { c };
        buf[y][x].color = if c == ' ' || p.pos.z > z {
            p.color
        } else {
            color
        };
        buf[y][x].z = if c == ' ' || p.pos.z > z { p.pos.z } else { z };
    }
    let mut s: String = String::from("");

    for row in buf.iter() {
        for b in row.iter() {
            s += if with_color { b.color } else { "" };
            s.push(b.c);
            s += if with_color { NEUTR } else { "" };
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn main() {
    let mut cubes = Vec::new();
    let mut rng = rand::thread_rng();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cubes n");
        return;
    }
    let ncubes = str::parse::<usize>(&args[1]).expect("n arg must be an unsigned int");
    let colors = [RED, GREEN, BLUE, YELLOW, CYAN, PURPLE];
    for _ in 0..ncubes {
        let pos = Vec3 {
            x: rng.gen::<f32>() * MAX_L,
            y: rng.gen::<f32>() * MAX_H,
            z: rng.gen::<f32>() * MAX_Z,
        };
        let v = Vec3 {
            x: rng.gen::<f32>() * V,
            y: rng.gen::<f32>() * V,
            z: rng.gen::<f32>() * V,
        };
        let a = Vec3 {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>(),
        };
        let alpha = Vec3 {
            x: rng.gen::<f32>() * A,
            y: rng.gen::<f32>() * A,
            z: rng.gen::<f32>() * A,
        };
        let idx: usize = rng.gen_range(0..6);
        let l: usize = rng.gen_range(5..10);
        let new_cube = Cube::new(colors[idx], l, pos, v, a, alpha);
        if !new_cube.will_collide(&cubes) {
            // TODO: try generating another one?
            cubes.push(new_cube);
        }
    }
    loop {
        print!("{}[2J", 27 as char);
        let mut pts: Vec<Point> = cubes.iter().flat_map(|c| c.roto_transl()).collect();
        display(&mut pts, true);
        let collisions: Vec<bool> = cubes.iter().map(|c| c.will_collide(&cubes)).collect();
        for (c, will_collide) in std::iter::zip(cubes.iter_mut(), collisions) {
            c.tick(will_collide);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
