use rand::Rng;
use std::time::Instant;

#[derive(PartialEq, PartialOrd, Debug)]
struct Vec3 {
    z: f32,
    x: f32,
    y: f32,
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Vec4 {
    z: f32,
    x: f32,
    y: f32,
    w: f32,
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Point {
    pos: Vec4,
    c: char,
    color: &'static str,
}

#[derive(Copy, Clone, Debug)]
struct Buf {
    c: char,
    color: &'static str,
    z: f32,
}

type Mat4 = [[f32; 4]; 4];

// No "small stuff far away"
fn get_ortho_proj(l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) -> Mat4 {
    [
        [2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)],
        [0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)],
        [0.0, 0.0, -2.0 / (f - n), -(f + n) / (f - n)],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// Far away stuff appears small
fn get_persp_proj(l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) -> Mat4 {
    [
        [2.0 * n / (r - l), 0.0, (r + l) / (r - l), 0.0],
        [0.0, 2.0 * n / (t - b), (t + b) / (t - b), 0.0],
        [0.0, 0.0, -(f + n) / (f - n), 2.0 * f * n / (f - n)],
        [0.0, 0.0, -1.0, 0.0],
    ]
}

fn get_rotate_x_mat4(a: f32) -> Mat4 {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, f32::cos(a), -f32::sin(a), 0.0],
        [0.0, f32::sin(a), f32::cos(a), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn get_rotate_y_mat4(a: f32) -> Mat4 {
    [
        [f32::cos(a), 0.0, f32::sin(a), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-f32::sin(a), 0.0, f32::cos(a), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn get_rotate_z_mat4(a: f32) -> Mat4 {
    [
        [f32::cos(a), -f32::sin(a), 0.0, 0.0],
        [f32::sin(a), f32::cos(a), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn get_trasl_mat4(x: f32, y: f32, z: f32) -> Mat4 {
    [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn mul_mv4(m: &Mat4, v: &Vec4) -> Vec4 {
    Vec4 {
        x: m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3] * v.w,
        y: m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3] * v.w,
        z: m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3] * v.w,
        w: m[3][0] * v.x + m[3][1] * v.y + m[3][2] * v.z + m[3][3] * v.w,
    }
}

fn apply(m: &Mat4, p: &Point) -> Point {
    let newpos = mul_mv4(m, &p.pos);
    Point {
        pos: newpos,
        c: p.c,
        color: p.color,
    }
}

fn mul_mm4(m1: &Mat4, m2: &Mat4) -> Mat4 {
    let mut r = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for (k, row2) in m2.iter().enumerate() {
                r[i][j] += m1[i][k] * row2[j];
            }
        }
    }
    r
}

fn get_world_m(a: &Vec3, t: &Vec3) -> Mat4 {
    let rx = get_rotate_x_mat4(a.x);
    let ry = get_rotate_y_mat4(a.y);
    let rz = get_rotate_z_mat4(a.z);
    let t = get_trasl_mat4(t.x, t.y, t.z);

    // t * rz * ry * rx
    mul_mm4(&t, &mul_mm4(&rz, &mul_mm4(&ry, &rx)))
}

fn get_view_matrix(a: &Vec3, t: &Vec3) -> Mat4 {
    let t = get_trasl_mat4(-t.x, -t.y, -t.z);
    let rx = get_rotate_x_mat4(-a.x);
    let ry = get_rotate_y_mat4(-a.y);
    let rz = get_rotate_z_mat4(-a.z);

    mul_mm4(&rx, &mul_mm4(&ry, &mul_mm4(&rz, &t)))
}

const S_H: usize = 80;
const S_W: usize = 200;

const MAX_L: f32 = 100.0;
const MIN_L: f32 = -100.0;
const MAX_H: f32 = 50.0;
const MIN_H: f32 = -50.0;
const MAX_Z: f32 = 20.0;
const MIN_Z: f32 = -20.0;

const V: f32 = 0.08; // units of space / milliseconds
const A: f32 = 0.01; // rad / milliseconds
const MIN_SIZE: usize = 5;
const MAX_SIZE: usize = 10;

const CAMERA_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 50.0,
};
const CAMERA_ANGLE: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

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
                    pos: Vec4 {
                        x: i as f32,
                        y: j as f32,
                        z: l as f32 / 2.0,
                        w: 1.0,
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
                    pos: Vec4 {
                        x: i as f32,
                        y: j as f32,
                        z: -(l as f32) / 2.0,
                        w: 1.0,
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
                    pos: Vec4 {
                        x: l as f32 / 2.0,
                        y: j as f32,
                        z: i as f32,
                        w: 1.0,
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
                    pos: Vec4 {
                        x: -(l as f32) / 2.0,
                        y: j as f32,
                        z: i as f32,
                        w: 1.0,
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
                    pos: Vec4 {
                        x: j as f32,
                        y: l as f32 / 2.0,
                        z: i as f32,
                        w: 1.0,
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
                    pos: Vec4 {
                        x: j as f32,
                        y: -(l as f32) / 2.0,
                        z: i as f32,
                        w: 1.0,
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
        let w = get_world_m(&self.a, &self.pos);
        let v = get_view_matrix(&CAMERA_ANGLE, &CAMERA_POS);
        let vw = mul_mm4(&v, &w);
        return self.points.iter().map(|p| apply(&vw, p)).collect();
    }
}

enum ProjT {
    ORTHO,
    PERSP,
}

fn display(points: &mut [Point], with_color: bool, proj_t: ProjT) {
    let mut buf: BufferT = [[Buf {
        c: ' ',
        color: NEUTR,
        z: 0.0,
    }; S_W]; S_H];

    let proj_mat = match proj_t {
        ProjT::ORTHO => get_ortho_proj(MIN_L, MAX_L, MIN_H, MAX_H, MIN_Z, MAX_Z),
        ProjT::PERSP => get_persp_proj(MIN_L, MAX_L, MIN_H, MAX_H, MIN_Z, MAX_Z),
    };
    let mut projected_points: Vec<Point> = match proj_t {
        ProjT::ORTHO => points.iter().map(|p| apply(&proj_mat, p)).collect(),
        ProjT::PERSP => points
            .iter()
            .map(|p| apply(&proj_mat, p))
            .map(|p| Point {
                pos: Vec4 {
                    x: p.pos.x / p.pos.w,
                    y: p.pos.y / p.pos.w,
                    z: p.pos.z / p.pos.w,
                    w: p.pos.w,
                },
                c: p.c,
                color: p.color,
            })
            .collect(),
    };
    projected_points.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());
    for p in projected_points {
        if f32::abs(p.pos.x) > 1.0 || f32::abs(p.pos.y) > 1.0 {
            continue;
        }
        // projection matrix and clipping puts us in [-1,1], we want to go
        // in x: [0, S_W] and y: [0, S_H]
        let x = ((p.pos.x + 1.0) * S_W as f32 / 2.0) as usize;
        let y = ((p.pos.y + 1.0) * S_H as f32 / 2.0) as usize;
        if x > S_W || y > S_H {
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
        let l: usize = rng.gen_range(MIN_SIZE..MAX_SIZE);
        let new_cube = Cube::new(colors[idx], l, pos, v, a, alpha);
        if !new_cube.will_collide(&cubes) {
            // TODO: try generating another one?
            cubes.push(new_cube);
        }
    }
    loop {
        print!("{}[2J", 27 as char);
        let mut pts: Vec<Point> = cubes.iter().flat_map(|c| c.roto_transl()).collect();
        display(&mut pts, true, ProjT::PERSP);
        let collisions: Vec<bool> = cubes.iter().map(|c| c.will_collide(&cubes)).collect();
        for (c, will_collide) in std::iter::zip(cubes.iter_mut(), collisions) {
            c.tick(will_collide);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
