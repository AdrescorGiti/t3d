use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fmt::Write as FmtWrite;
use std::io::{stdout, Write};
use std::ops::{Add, Mul, Sub};
use std::time::{Duration, Instant};

const SHADE_RAMP: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Clone, Copy, Default)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    #[inline]
    fn cross(self, o: Self) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    #[inline]
    fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    fn norm(self) -> Self {
        let l = self.len();
        if l > 1e-6 {
            self * (1.0 / l)
        } else {
            Self::new(0.0, 0.0, 1.0)
        }
    }

    #[inline]
    fn rot(self, rx: f32, ry: f32, rz: f32) -> Self {
        let (sx, cx) = rx.sin_cos();
        let y1 = self.y * cx - self.z * sx;
        let z1 = self.y * sx + self.z * cx;

        let (sy, cy) = ry.sin_cos();
        let x2 = self.x * cy + z1 * sy;
        let z2 = -self.x * sy + z1 * cy;

        let (sz, cz) = rz.sin_cos();
        Self {
            x: x2 * cz - y1 * sz,
            y: x2 * sz + y1 * cz,
            z: z2,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

#[derive(Clone, Copy)]
struct Face {
    idx: [usize; 3],
    tone: f32,
}

struct Mesh {
    name: &'static str,
    verts: Vec<Vec3>,
    faces: Vec<Face>,
    edges: Vec<(usize, usize)>,
}

impl Mesh {
    fn orient_from_origin(&mut self) {
        for f in &mut self.faces {
            let a = self.verts[f.idx[0]];
            let b = self.verts[f.idx[1]];
            let c = self.verts[f.idx[2]];
            let center = (a + b + c) * (1.0 / 3.0);
            let n = (b - a).cross(c - a);
            if n.dot(center) < 0.0 {
                f.idx.swap(1, 2);
            }
        }
    }

    fn cube() -> Self {
        let s = 1.05;
        let verts = vec![
            Vec3::new(-s, -s, -s),
            Vec3::new(s, -s, -s),
            Vec3::new(s, s, -s),
            Vec3::new(-s, s, -s),
            Vec3::new(-s, -s, s),
            Vec3::new(s, -s, s),
            Vec3::new(s, s, s),
            Vec3::new(-s, s, s),
        ];
        let quads = [
            ([0, 1, 2, 3], 1.0),
            ([4, 5, 6, 7], 1.0),
            ([0, 4, 7, 3], 0.85),
            ([1, 5, 6, 2], 0.85),
            ([0, 1, 5, 4], 0.7),
            ([3, 2, 6, 7], 0.7),
        ];
        let mut faces = Vec::with_capacity(12);
        for (q, t) in quads {
            faces.push(Face { idx: [q[0], q[1], q[2]], tone: t });
            faces.push(Face { idx: [q[0], q[2], q[3]], tone: t });
        }
        let edges = vec![
            (0, 1), (1, 2), (2, 3), (3, 0),
            (4, 5), (5, 6), (6, 7), (7, 4),
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];
        let mut m = Self { name: "Cube", verts, faces, edges };
        m.orient_from_origin();
        m
    }

    fn pyramid() -> Self {
        let s = 1.25;
        let h = 1.35;
        let verts = vec![
            Vec3::new(0.0, -h, 0.0),
            Vec3::new(-s, h * 0.65, -s),
            Vec3::new(s, h * 0.65, -s),
            Vec3::new(s, h * 0.65, s),
            Vec3::new(-s, h * 0.65, s),
        ];
        let faces = vec![
            Face { idx: [0, 1, 2], tone: 1.0 },
            Face { idx: [0, 2, 3], tone: 0.82 },
            Face { idx: [0, 3, 4], tone: 1.0 },
            Face { idx: [0, 4, 1], tone: 0.82 },
            Face { idx: [1, 2, 3], tone: 0.65 },
            Face { idx: [1, 3, 4], tone: 0.65 },
        ];
        let edges = vec![
            (0, 1), (0, 2), (0, 3), (0, 4),
            (1, 2), (2, 3), (3, 4), (4, 1),
        ];
        let mut m = Self { name: "Pyramid", verts, faces, edges };
        m.orient_from_origin();
        m
    }

    fn globe() -> Self {
        let rings = 12usize;
        let sectors = 24usize;
        let r = 1.45;
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut edges = Vec::new();

        for i in 0..=rings {
            let v = i as f32 / rings as f32;
            let phi = v * std::f32::consts::PI;
            for j in 0..=sectors {
                let u = j as f32 / sectors as f32;
                let theta = u * std::f32::consts::TAU;
                verts.push(Vec3::new(
                    r * phi.sin() * theta.cos(),
                    -r * phi.cos(),
                    r * phi.sin() * theta.sin(),
                ));
            }
        }

        let stride = sectors + 1;
        for i in 0..rings {
            for j in 0..sectors {
                let a = i * stride + j;
                let b = a + 1;
                let c = (i + 1) * stride + j;
                let d = c + 1;
                let checker = ((i / 2) + (j / 2)) % 2 == 0;
                let tone = if checker { 1.0 } else { 0.58 };

                if i != 0 {
                    faces.push(Face { idx: [a, d, b], tone });
                }
                if i != rings - 1 {
                    faces.push(Face { idx: [a, c, d], tone });
                }
                if i > 0 && i % 2 == 0 {
                    edges.push((a, b));
                }
                if j % 2 == 0 {
                    edges.push((a, c));
                }
            }
        }

        let mut m = Self { name: "Globe", verts, faces, edges };
        m.orient_from_origin();
        m
    }

    fn ruby() -> Self {
        let n = 8usize;
        let mut verts = Vec::new();

        verts.push(Vec3::new(0.0, -0.85, 0.0));
        verts.push(Vec3::new(0.0, 1.35, 0.0));

        let rings = [
            (-0.85, 0.62, 0.0),
            (-0.35, 1.28, std::f32::consts::PI / n as f32),
            (-0.12, 1.28, std::f32::consts::PI / n as f32),
        ];

        for &(y, rad, phase) in &rings {
            for i in 0..n {
                let a = phase + (i as f32) * std::f32::consts::TAU / (n as f32);
                verts.push(Vec3::new(rad * a.cos(), y, rad * a.sin()));
            }
        }

        let top_center = 0;
        let culet = 1;
        let r0 = 2;
        let r1 = r0 + n;
        let r2 = r1 + n;

        let mut faces = Vec::new();
        let mut edges = Vec::new();

        for i in 0..n {
            let ni = (i + 1) % n;
            let t0 = r0 + i;
            let t1 = r0 + ni;
            let c0 = r1 + i;
            let c1 = r1 + ni;
            let g0 = r2 + i;
            let g1 = r2 + ni;

            let alt = if i % 2 == 0 { 1.0 } else { 0.78 };

            faces.push(Face { idx: [top_center, t0, t1], tone: 0.95 });
            faces.push(Face { idx: [t0, c0, t1], tone: alt });
            faces.push(Face { idx: [t1, c0, c1], tone: 1.08 });
            faces.push(Face { idx: [c0, g0, c1], tone: 0.85 });
            faces.push(Face { idx: [c1, g0, g1], tone: 0.75 });
            faces.push(Face { idx: [g0, culet, g1], tone: alt });

            edges.push((t0, t1));
            edges.push((c0, c1));
            edges.push((g0, g1));
            edges.push((t0, c0));
            edges.push((t1, c0));
            edges.push((c0, g0));
            edges.push((g0, culet));
        }

        let mut m = Self { name: "Ruby", verts, faces, edges };
        m.orient_from_origin();
        m
    }

    fn torus() -> Self {
        let maj_seg = 24usize;
        let min_seg = 12usize;
        let r_maj = 1.05;
        let r_min = 0.42;
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut edges = Vec::new();

        for i in 0..maj_seg {
            let u = (i as f32 / maj_seg as f32) * std::f32::consts::TAU;
            let (su, cu) = u.sin_cos();
            let center = Vec3::new(r_maj * cu, 0.0, r_maj * su);

            for j in 0..min_seg {
                let v = (j as f32 / min_seg as f32) * std::f32::consts::TAU;
                let (sv, cv) = v.sin_cos();
                let p = Vec3::new(
                    (r_maj + r_min * cv) * cu,
                    r_min * sv,
                    (r_maj + r_min * cv) * su,
                );
                verts.push(p);

                let ni = (i + 1) % maj_seg;
                let nj = (j + 1) % min_seg;
                let a = i * min_seg + j;
                let b = ni * min_seg + j;
                let c = ni * min_seg + nj;
                let d = i * min_seg + nj;

                let tone = if (i + j) % 2 == 0 { 1.0 } else { 0.72 };
                let _ = center;
                faces.push(Face { idx: [a, b, c], tone });
                faces.push(Face { idx: [a, c, d], tone });

                if j % 2 == 0 {
                    edges.push((a, b));
                }
                if i % 2 == 0 {
                    edges.push((a, d));
                }
            }
        }

        for f in &mut faces {
            let i = f.idx[0] / min_seg;
            let u = (i as f32 / maj_seg as f32) * std::f32::consts::TAU;
            let ring_c = Vec3::new(r_maj * u.cos(), 0.0, r_maj * u.sin());
            let a = verts[f.idx[0]];
            let b = verts[f.idx[1]];
            let c = verts[f.idx[2]];
            let tc = (a + b + c) * (1.0 / 3.0);
            if (b - a).cross(c - a).dot(tc - ring_c) < 0.0 {
                f.idx.swap(1, 2);
            }
        }

        Self { name: "Torus", verts, faces, edges }
    }

    fn octahedron() -> Self {
        let verts = vec![
            Vec3::new(0.0, -1.5, 0.0),
            Vec3::new(0.0, 1.5, 0.0),
            Vec3::new(-1.15, 0.0, -1.15),
            Vec3::new(1.15, 0.0, -1.15),
            Vec3::new(1.15, 0.0, 1.15),
            Vec3::new(-1.15, 0.0, 1.15),
        ];
        let faces = vec![
            Face { idx: [0, 2, 3], tone: 1.0 },
            Face { idx: [0, 3, 4], tone: 0.8 },
            Face { idx: [0, 4, 5], tone: 1.0 },
            Face { idx: [0, 5, 2], tone: 0.8 },
            Face { idx: [1, 3, 2], tone: 0.9 },
            Face { idx: [1, 4, 3], tone: 0.7 },
            Face { idx: [1, 5, 4], tone: 0.9 },
            Face { idx: [1, 2, 5], tone: 0.7 },
        ];
        let edges = vec![
            (0, 2), (0, 3), (0, 4), (0, 5),
            (1, 2), (1, 3), (1, 4), (1, 5),
            (2, 3), (3, 4), (4, 5), (5, 2),
        ];
        let mut m = Self { name: "Octahedron", verts, faces, edges };
        m.orient_from_origin();
        m
    }

    fn icosahedron() -> Self {
        let phi = (1.0 + 5.0_f32.sqrt()) * 0.5;
        let s = 0.82;
        let raw = [
            (-1.0, phi, 0.0), (1.0, phi, 0.0), (-1.0, -phi, 0.0), (1.0, -phi, 0.0),
            (0.0, -1.0, phi), (0.0, 1.0, phi), (0.0, -1.0, -phi), (0.0, 1.0, -phi),
            (phi, 0.0, -1.0), (phi, 0.0, 1.0), (-phi, 0.0, -1.0), (-phi, 0.0, 1.0),
        ];
        let verts: Vec<Vec3> = raw.iter().map(|&(x, y, z)| Vec3::new(x * s, y * s, z * s)).collect();
        let idx_list = [
            [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
            [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
            [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
            [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
        ];
        let mut faces = Vec::new();
        let mut edges = Vec::new();
        for (k, tri) in idx_list.iter().enumerate() {
            let tone = if k % 2 == 0 { 1.0 } else { 0.78 };
            faces.push(Face { idx: *tri, tone });
            for &(u, v) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                let e = if u < v { (u, v) } else { (v, u) };
                if !edges.contains(&e) {
                    edges.push(e);
                }
            }
        }
        let mut m = Self { name: "Icosahedron", verts, faces, edges };
        m.orient_from_origin();
        m
    }

    fn trefoil() -> Self {
        let steps = 64usize;
        let sides = 8usize;
        let tube_r = 0.24;
        let scale = 0.42;

        let sample = |t: f32| -> Vec3 {
            Vec3::new(
                (t.sin() + 2.0 * (2.0 * t).sin()) * scale,
                (t.cos() - 2.0 * (2.0 * t).cos()) * scale,
                -(3.0 * t).sin() * scale * 1.2,
            )
        };

        let mut centers = Vec::with_capacity(steps);
        let mut verts = Vec::with_capacity(steps * sides);

        for i in 0..steps {
            let t = (i as f32 / steps as f32) * std::f32::consts::TAU;
            let p = sample(t);
            let p_next = sample(t + 0.01);
            let tang = (p_next - p).norm();
            let up = if tang.y.abs() < 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            let n1 = tang.cross(up).norm();
            let n2 = tang.cross(n1).norm();
            centers.push(p);

            for j in 0..sides {
                let a = (j as f32 / sides as f32) * std::f32::consts::TAU;
                let offset = n1 * (a.cos() * tube_r) + n2 * (a.sin() * tube_r);
                verts.push(p + offset);
            }
        }

        let mut faces = Vec::new();
        let mut edges = Vec::new();

        for i in 0..steps {
            let ni = (i + 1) % steps;
            for j in 0..sides {
                let nj = (j + 1) % sides;
                let a = i * sides + j;
                let b = ni * sides + j;
                let c = ni * sides + nj;
                let d = i * sides + nj;
                let tone = if (i / 2 + j) % 2 == 0 { 1.0 } else { 0.72 };

                let mut f1 = Face { idx: [a, b, c], tone };
                let mut f2 = Face { idx: [a, c, d], tone };

                let tc1 = (verts[a] + verts[b] + verts[c]) * (1.0 / 3.0);
                if (verts[b] - verts[a]).cross(verts[c] - verts[a]).dot(tc1 - centers[i]) < 0.0 {
                    f1.idx.swap(1, 2);
                }
                let tc2 = (verts[a] + verts[c] + verts[d]) * (1.0 / 3.0);
                if (verts[c] - verts[a]).cross(verts[d] - verts[a]).dot(tc2 - centers[i]) < 0.0 {
                    f2.idx.swap(1, 2);
                }

                faces.push(f1);
                faces.push(f2);

                if j % 2 == 0 {
                    edges.push((a, b));
                }
                if i % 4 == 0 {
                    edges.push((a, d));
                }
            }
        }

        Self { name: "Trefoil Knot", verts, faces, edges }
    }

    fn helix() -> Self {
        let steps = 36usize;
        let rad = 0.95;
        let height = 2.8;
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut edges = Vec::new();

        for i in 0..steps {
            let f = i as f32 / (steps - 1) as f32;
            let y = (f - 0.5) * height;
            let a = f * std::f32::consts::TAU * 2.0;

            let p1 = Vec3::new(rad * a.cos(), y, rad * a.sin());
            let p2 = Vec3::new(rad * (a + std::f32::consts::PI).cos(), y, rad * (a + std::f32::consts::PI).sin());

            let base = verts.len();
            let d = 0.11;
            for &p in &[p1, p2] {
                verts.push(p + Vec3::new(-d, -d, 0.0));
                verts.push(p + Vec3::new(d, -d, 0.0));
                verts.push(p + Vec3::new(d, d, 0.0));
                verts.push(p + Vec3::new(-d, d, 0.0));
            }

            if i > 0 {
                let prev = base - 8;
                for strand in 0..2 {
                    let s0 = prev + strand * 4;
                    let s1 = base + strand * 4;
                    edges.push((s0, s1));
                    edges.push((s0 + 2, s1 + 2));
                    for k in 0..4 {
                        let nk = (k + 1) % 4;
                        faces.push(Face { idx: [s0 + k, s1 + k, s1 + nk], tone: 1.0 });
                        faces.push(Face { idx: [s0 + k, s1 + nk, s0 + nk], tone: 0.8 });
                    }
                }
            }

            if i % 3 == 0 {
                let a0 = base;
                let b0 = base + 4;
                edges.push((a0, b0));
                edges.push((a0 + 2, b0 + 2));
                faces.push(Face { idx: [a0, b0, b0 + 2], tone: 0.65 });
                faces.push(Face { idx: [a0, b0 + 2, a0 + 2], tone: 0.65 });
            }
        }

        Self { name: "DNA Helix", verts, faces, edges }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Hybrid,
    Solid,
    Braille,
    Wire,
    Points,
}

impl Mode {
    fn next(self) -> Self {
        match self {
            Self::Hybrid => Self::Solid,
            Self::Solid => Self::Braille,
            Self::Braille => Self::Wire,
            Self::Wire => Self::Points,
            Self::Points => Self::Hybrid,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Hybrid => "Hybrid",
            Self::Solid => "Solid",
            Self::Braille => "Braille",
            Self::Wire => "Wire",
            Self::Points => "Points",
        }
    }
}

#[derive(Clone, Copy)]
struct Theme {
    name: &'static str,
    low: (u8, u8, u8),
    mid: (u8, u8, u8),
    high: (u8, u8, u8),
    spec: (u8, u8, u8),
}

const THEMES: &[Theme] = &[
    Theme {
        name: "Crimson Ruby",
        low: (55, 8, 22),
        mid: (215, 25, 65),
        high: (255, 95, 130),
        spec: (255, 235, 240),
    },
    Theme {
        name: "Cyber Neon",
        low: (25, 15, 75),
        mid: (0, 210, 255),
        high: (255, 50, 210),
        spec: (240, 255, 255),
    },
    Theme {
        name: "Emerald Matrix",
        low: (8, 45, 18),
        mid: (25, 185, 65),
        high: (130, 255, 155),
        spec: (230, 255, 235),
    },
    Theme {
        name: "Amber CRT",
        low: (60, 25, 5),
        mid: (220, 120, 15),
        high: (255, 200, 65),
        spec: (255, 250, 220),
    },
    Theme {
        name: "Argon Frost",
        low: (15, 35, 75),
        mid: (65, 145, 255),
        high: (165, 225, 255),
        spec: (255, 255, 255),
    },
];

impl Theme {
    fn sample(&self, lum: f32, spec: f32) -> (u8, u8, u8) {
        let l = lum.clamp(0.0, 1.0);
        let (r, g, b) = if l < 0.5 {
            let t = l * 2.0;
            (
                self.low.0 as f32 + (self.mid.0 as f32 - self.low.0 as f32) * t,
                self.low.1 as f32 + (self.mid.1 as f32 - self.low.1 as f32) * t,
                self.low.2 as f32 + (self.mid.2 as f32 - self.low.2 as f32) * t,
            )
        } else {
            let t = (l - 0.5) * 2.0;
            (
                self.mid.0 as f32 + (self.high.0 as f32 - self.mid.0 as f32) * t,
                self.mid.1 as f32 + (self.high.1 as f32 - self.mid.1 as f32) * t,
                self.mid.2 as f32 + (self.high.2 as f32 - self.mid.2 as f32) * t,
            )
        };
        let s = spec.clamp(0.0, 1.0);
        (
            (r + (self.spec.0 as f32 - r) * s) as u8,
            (g + (self.spec.1 as f32 - g) * s) as u8,
            (b + (self.spec.2 as f32 - b) * s) as u8,
        )
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn f32(&mut self) -> f32 {
        (self.next() & 0xFFFFFF) as f32 / 16777216.0
    }

    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.f32()
    }
}

struct Star {
    pos: Vec3,
    prev_z: f32,
}

struct Buffer {
    w: usize,
    h: usize,
    ch: Vec<char>,
    fg: Vec<(u8, u8, u8)>,
    zbuf: Vec<f32>,
    braille: Vec<u8>,
    braille_z: Vec<f32>,
}

impl Buffer {
    fn new(w: usize, h: usize) -> Self {
        let n = w * h;
        Self {
            w,
            h,
            ch: vec![' '; n],
            fg: vec![(0, 0, 0); n],
            zbuf: vec![f32::INFINITY; n],
            braille: vec![0; n],
            braille_z: vec![f32::INFINITY; n],
        }
    }

    fn resize(&mut self, w: usize, h: usize) {
        if self.w != w || self.h != h {
            *self = Self::new(w, h);
        }
    }

    fn clear(&mut self) {
        self.ch.fill(' ');
        self.fg.fill((0, 0, 0));
        self.zbuf.fill(f32::INFINITY);
        self.braille.fill(0);
        self.braille_z.fill(f32::INFINITY);
    }

    #[inline]
    fn put(&mut self, x: i32, y: i32, z: f32, c: char, color: (u8, u8, u8)) {
        if x >= 0 && x < self.w as i32 && y >= 0 && y < self.h as i32 {
            let idx = y as usize * self.w + x as usize;
            if z <= self.zbuf[idx] {
                self.zbuf[idx] = z;
                self.ch[idx] = c;
                self.fg[idx] = color;
            }
        }
    }

    #[inline]
    fn put_subpixel(&mut self, sx: i32, sy: i32, z: f32, color: (u8, u8, u8)) {
        if sx < 0 || sy < 0 {
            return;
        }
        let cx = (sx / 2) as usize;
        let cy = (sy / 4) as usize;
        if cx >= self.w || cy >= self.h {
            return;
        }
        let dx = (sx % 2) as usize;
        let dy = (sy % 4) as usize;
        let bit = match (dx, dy) {
            (0, 0) => 0x01,
            (0, 1) => 0x02,
            (0, 2) => 0x04,
            (1, 0) => 0x08,
            (1, 1) => 0x10,
            (1, 2) => 0x20,
            (0, 3) => 0x40,
            (1, 3) => 0x80,
            _ => 0,
        };
        let idx = cy * self.w + cx;
        self.braille[idx] |= bit;
        if z <= self.braille_z[idx] {
            self.braille_z[idx] = z;
            self.fg[idx] = color;
        }
    }

    fn flush_braille(&mut self) {
        for i in 0..self.braille.len() {
            if self.braille[i] != 0 && self.braille_z[i] <= self.zbuf[i] {
                self.zbuf[i] = self.braille_z[i];
                self.ch[i] = char::from_u32(0x2800 + self.braille[i] as u32).unwrap_or(' ');
            }
        }
    }

    fn line_3d(
        &mut self,
        p0: (i32, i32, f32),
        p1: (i32, i32, f32),
        c: char,
        col: (u8, u8, u8),
        dim_col: (u8, u8, u8),
        depth_test: bool,
    ) {
        let (mut x0, mut y0, z0) = p0;
        let (x1, y1, z1) = p1;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let steps = dx.max(-dy).max(1) as f32;
        let mut step = 0.0;

        loop {
            let t = step / steps;
            let z = z0 + (z1 - z0) * t;
            if x0 >= 0 && x0 < self.w as i32 && y0 >= 0 && y0 < self.h as i32 {
                let idx = y0 as usize * self.w + x0 as usize;
                if z <= self.zbuf[idx] + 0.08 {
                    self.zbuf[idx] = z;
                    self.ch[idx] = c;
                    self.fg[idx] = col;
                } else if !depth_test && self.ch[idx] == ' ' {
                    self.ch[idx] = '.';
                    self.fg[idx] = dim_col;
                }
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
            step += 1.0;
        }
    }

    fn line_braille(
        &mut self,
        p0: (i32, i32, f32),
        p1: (i32, i32, f32),
        col: (u8, u8, u8),
    ) {
        let (mut x0, mut y0, z0) = p0;
        let (x1, y1, z1) = p1;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let steps = dx.max(-dy).max(1) as f32;
        let mut step = 0.0;

        loop {
            let t = step / steps;
            let z = z0 + (z1 - z0) * t;
            self.put_subpixel(x0, y0, z, col);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
            step += 1.0;
        }
    }

    fn triangle(
        &mut self,
        v0: (f32, f32, f32),
        v1: (f32, f32, f32),
        v2: (f32, f32, f32),
        ch: char,
        col: (u8, u8, u8),
    ) {
        let min_x = v0.0.min(v1.0).min(v2.0).floor().max(0.0) as i32;
        let max_x = v0.0.max(v1.0).max(v2.0).ceil().min(self.w as f32 - 1.0) as i32;
        let min_y = v0.1.min(v1.1).min(v2.1).floor().max(0.0) as i32;
        let max_y = v0.1.max(v1.1).max(v2.1).ceil().min(self.h as f32 - 1.0) as i32;

        let edge = |a: (f32, f32, f32), b: (f32, f32, f32), px: f32, py: f32| -> f32 {
            (px - a.0) * (b.1 - a.1) - (py - a.1) * (b.0 - a.0)
        };

        let area = edge(v0, v1, v2.0, v2.1);
        if area.abs() < 1e-4 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;
                let w0 = edge(v1, v2, px, py) / area;
                let w1 = edge(v2, v0, px, py) / area;
                let w2 = edge(v0, v1, px, py) / area;

                if w0 >= -0.001 && w1 >= -0.001 && w2 >= -0.001 {
                    let z = w0 * v0.2 + w1 * v1.2 + w2 * v2.2;
                    self.put(x, y, z, ch, col);
                }
            }
        }
    }

    fn text(&mut self, x: usize, y: usize, s: &str, col: (u8, u8, u8)) {
        if y >= self.h {
            return;
        }
        for (i, c) in s.chars().enumerate() {
            let px = x + i;
            if px < self.w {
                let idx = y * self.w + px;
                self.ch[idx] = c;
                self.fg[idx] = col;
                self.zbuf[idx] = -100.0;
            }
        }
    }
}

fn project(v: Vec3, w: f32, h: f32, fov: f32, dist: f32) -> (f32, f32, f32) {
    let z = (v.z + dist).max(0.2);
    let scale = fov / z;
    (w * 0.5 + v.x * scale * 2.1, h * 0.5 + v.y * scale, z)
}

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, Hide, EnableMouseCapture, Clear(ClearType::All))?;

    let meshes = vec![
        Mesh::cube(),
        Mesh::pyramid(),
        Mesh::globe(),
        Mesh::ruby(),
        Mesh::torus(),
        Mesh::octahedron(),
        Mesh::icosahedron(),
        Mesh::trefoil(),
        Mesh::helix(),
    ];

    let mut rng = Rng(0xC0DEC0FFEE123456);
    let mut stars: Vec<Star> = (0..180)
        .map(|_| {
            let z = rng.range(1.0, 22.0);
            Star {
                pos: Vec3::new(rng.range(-25.0, 25.0), rng.range(-14.0, 14.0), z),
                prev_z: z,
            }
        })
        .collect();

    let mut mesh_idx = 3usize;
    let mut mode = Mode::Hybrid;
    let mut theme_idx = 0usize;
    let mut rx = 0.35_f32;
    let mut ry = 0.65_f32;
    let mut rz = 0.15_f32;
    let mut dist = 4.2_f32;
    let mut paused = false;
    let mut star_mode = 1u8;
    let mut cull = true;
    let mut orbit_light = true;
    let mut pulse = false;
    let mut glitch = false;
    let mut show_hud = true;
    let mut drag_prev: Option<(u16, u16)> = None;

    let (tw, th) = terminal::size()?;
    let mut buf = Buffer::new(tw as usize, th as usize);
    let mut frame_str = String::with_capacity(tw as usize * th as usize * 16);

    let start_clock = Instant::now();
    let mut last_tick = Instant::now();
    let mut fps_accum = 0u32;
    let mut fps_display = 60u32;
    let mut fps_timer = Instant::now();

    loop {
        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(k) if k.kind == KeyEventKind::Press => match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        execute!(out, DisableMouseCapture, Show, LeaveAlternateScreen)?;
                        terminal::disable_raw_mode()?;
                        return Ok(());
                    }
                    KeyCode::Char(c @ '1'..='9') => {
                        let idx = (c as usize) - ('1' as usize);
                        if idx < meshes.len() {
                            mesh_idx = idx;
                        }
                    }
                    KeyCode::Tab | KeyCode::Char(']') => mesh_idx = (mesh_idx + 1) % meshes.len(),
                    KeyCode::Char('[') => mesh_idx = (mesh_idx + meshes.len() - 1) % meshes.len(),
                    KeyCode::Char('m') | KeyCode::Char('M') => mode = mode.next(),
                    KeyCode::Char('c') | KeyCode::Char('C') => theme_idx = (theme_idx + 1) % THEMES.len(),
                    KeyCode::Char('b') | KeyCode::Char('B') => star_mode = (star_mode + 1) % 3,
                    KeyCode::Char('x') | KeyCode::Char('X') => cull = !cull,
                    KeyCode::Char('l') | KeyCode::Char('L') => orbit_light = !orbit_light,
                    KeyCode::Char('p') | KeyCode::Char('P') => pulse = !pulse,
                    KeyCode::Char('g') | KeyCode::Char('G') => glitch = !glitch,
                    KeyCode::Char('h') | KeyCode::Char('H') => show_hud = !show_hud,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        rx = 0.35;
                        ry = 0.65;
                        rz = 0.15;
                        dist = 4.2;
                    }
                    KeyCode::Up | KeyCode::Char('w') => rx -= 0.14,
                    KeyCode::Down | KeyCode::Char('s') => rx += 0.14,
                    KeyCode::Left | KeyCode::Char('a') => ry -= 0.14,
                    KeyCode::Right | KeyCode::Char('d') => ry += 0.14,
                    KeyCode::Char('+') | KeyCode::Char('=') => dist = (dist - 0.25).max(1.8),
                    KeyCode::Char('-') | KeyCode::Char('_') => dist = (dist + 0.25).min(11.0),
                    _ => {}
                },
                Event::Mouse(me) => match me.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        drag_prev = Some((me.column, me.row));
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        if let Some((px, py)) = drag_prev {
                            let dx = me.column as f32 - px as f32;
                            let dy = me.row as f32 - py as f32;
                            ry += dx * 0.04;
                            rx += dy * 0.07;
                        }
                        drag_prev = Some((me.column, me.row));
                    }
                    MouseEventKind::Up(MouseButton::Left) => drag_prev = None,
                    MouseEventKind::ScrollUp => dist = (dist - 0.25).max(1.8),
                    MouseEventKind::ScrollDown => dist = (dist + 0.25).min(11.0),
                    _ => {}
                },
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32().min(0.1);
        last_tick = now;
        let elapsed = start_clock.elapsed().as_secs_f32();

        fps_accum += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps_display = fps_accum;
            fps_accum = 0;
            fps_timer = Instant::now();
        }

        if !paused && drag_prev.is_none() {
            rx += 0.65 * dt;
            ry += 1.05 * dt;
            rz += 0.35 * dt;
        }

        let (cw, ch) = terminal::size()?;
        buf.resize(cw as usize, ch as usize);
        buf.clear();

        let wf = buf.w as f32;
        let hf = buf.h as f32;
        let fov = hf * 0.92;
        let theme = &THEMES[theme_idx];

        if star_mode > 0 {
            let speed = if star_mode == 2 { 22.0 } else { 4.5 };
            for st in &mut stars {
                st.prev_z = st.pos.z;
                if !paused {
                    st.pos.z -= speed * dt;
                }
                if st.pos.z <= 0.4 {
                    st.pos.x = rng.range(-25.0, 25.0);
                    st.pos.y = rng.range(-14.0, 14.0);
                    st.pos.z = 22.0;
                    st.prev_z = 22.0;
                }
                let (sx, sy, _) = project(st.pos, wf, hf, fov * 0.55, 0.0);
                let bright = (1.0 - st.pos.z / 22.0).clamp(0.15, 1.0);
                let col = (
                    (190.0 * bright) as u8,
                    (210.0 * bright) as u8,
                    (255.0 * bright) as u8,
                );
                if star_mode == 2 {
                    let prev_p = Vec3::new(st.pos.x, st.pos.y, st.prev_z + 1.4);
                    let (px, py, _) = project(prev_p, wf, hf, fov * 0.55, 0.0);
                    buf.line_3d(
                        (sx as i32, sy as i32, 90.0),
                        (px as i32, py as i32, 90.0),
                        '-',
                        col,
                        col,
                        true,
                    );
                } else {
                    let sym = if st.pos.z < 7.0 {
                        '+'
                    } else if st.pos.z < 14.0 {
                        '*'
                    } else {
                        '.'
                    };
                    buf.put(sx as i32, sy as i32, 90.0, sym, col);
                }
            }
        }

        let light_dir = if orbit_light {
            Vec3::new((elapsed * 1.6).cos() * 0.8, -0.55, -0.75 + (elapsed * 1.6).sin() * 0.35).norm()
        } else {
            Vec3::new(0.45, -0.65, -0.75).norm()
        };
        let view_dir = Vec3::new(0.0, 0.0, -1.0);
        let half_vec = (light_dir * -1.0 + view_dir).norm();

        let mesh = &meshes[mesh_idx];
        let mut t_verts = Vec::with_capacity(mesh.verts.len());
        let mut p_verts = Vec::with_capacity(mesh.verts.len());

        for &v in &mesh.verts {
            let mut mv = v;
            if pulse {
                let wave = 1.0 + 0.16 * (elapsed * 4.0 + v.y * 2.5).sin();
                mv = mv * wave;
            }
            let rv = mv.rot(rx, ry, rz);
            t_verts.push(rv);
            p_verts.push(project(rv, wf, hf, fov, dist));
        }

        if mode == Mode::Solid || mode == Mode::Hybrid {
            for f in &mesh.faces {
                let a = t_verts[f.idx[0]];
                let b = t_verts[f.idx[1]];
                let c = t_verts[f.idx[2]];
                let n = (b - a).cross(c - a).norm();
                let center = (a + b + c) * (1.0 / 3.0);
                let to_cam = (Vec3::new(0.0, 0.0, -dist) - center).norm();

                if cull && n.dot(to_cam) <= 0.0 {
                    continue;
                }

                let face_n = if n.dot(to_cam) < 0.0 { n * -1.0 } else { n };
                let diff = face_n.dot(light_dir * -1.0).max(0.0);
                let spec = face_n.dot(half_vec).max(0.0).powi(16);
                let lum = ((0.16 + diff * 0.84) * f.tone).clamp(0.05, 1.0);
                let ramp_idx = ((lum * (SHADE_RAMP.len() - 1) as f32) as usize).min(SHADE_RAMP.len() - 1);
                let ch_sym = SHADE_RAMP[ramp_idx] as char;
                let col = theme.sample(lum, spec * 0.85);

                buf.triangle(p_verts[f.idx[0]], p_verts[f.idx[1]], p_verts[f.idx[2]], ch_sym, col);
            }
        }

        match mode {
            Mode::Hybrid => {
                for &(u, v) in &mesh.edges {
                    let p0 = p_verts[u];
                    let p1 = p_verts[v];
                    buf.line_3d(
                        (p0.0 as i32, p0.1 as i32, p0.2 - 0.03),
                        (p1.0 as i32, p1.1 as i32, p1.2 - 0.03),
                        '#',
                        theme.spec,
                        theme.low,
                        true,
                    );
                }
            }
            Mode::Wire => {
                for &(u, v) in &mesh.edges {
                    let p0 = p_verts[u];
                    let p1 = p_verts[v];
                    let avg_z = (t_verts[u].z + t_verts[v].z) * 0.5;
                    let front = avg_z <= 0.15;
                    let lum = (1.0 - (avg_z + 1.6) / 3.2).clamp(0.2, 1.0);
                    let col = theme.sample(lum, if front { 0.35 } else { 0.0 });
                    let sym = if front { '@' } else { ':' };
                    buf.line_3d(
                        (p0.0 as i32, p0.1 as i32, p0.2),
                        (p1.0 as i32, p1.1 as i32, p1.2),
                        sym,
                        col,
                        theme.low,
                        cull,
                    );
                }
            }
            Mode::Braille => {
                let bw = wf * 2.0;
                let bh = hf * 4.0;
                let bfov = bh * 0.92;
                for &(u, v) in &mesh.edges {
                    let a = t_verts[u];
                    let b = t_verts[v];
                    let avg_z = (a.z + b.z) * 0.5;
                    if cull && avg_z > 0.55 {
                        continue;
                    }
                    let pa = (
                        bw * 0.5 + a.x * (bfov / (a.z + dist).max(0.2)) * 1.05,
                        bh * 0.5 + a.y * (bfov / (a.z + dist).max(0.2)),
                        a.z + dist,
                    );
                    let pb = (
                        bw * 0.5 + b.x * (bfov / (b.z + dist).max(0.2)) * 1.05,
                        bh * 0.5 + b.y * (bfov / (b.z + dist).max(0.2)),
                        b.z + dist,
                    );
                    let lum = (1.0 - (avg_z + 1.6) / 3.2).clamp(0.25, 1.0);
                    let col = theme.sample(lum, if avg_z < -0.2 { 0.45 } else { 0.0 });
                    buf.line_braille(
                        (pa.0 as i32, pa.1 as i32, pa.2),
                        (pb.0 as i32, pb.1 as i32, pb.2),
                        col,
                    );
                }
                buf.flush_braille();
            }
            Mode::Points => {
                for (i, &rv) in t_verts.iter().enumerate() {
                    let p = p_verts[i];
                    let lum = (1.0 - (rv.z + 1.6) / 3.2).clamp(0.2, 1.0);
                    let col = theme.sample(lum, 0.5);
                    let sym = if rv.z < -0.3 { '@' } else if rv.z < 0.4 { 'o' } else { '.' };
                    buf.put(p.0 as i32, p.1 as i32, p.2, sym, col);

                    let tip = project(rv + rv.norm() * 0.22, wf, hf, fov, dist);
                    buf.line_3d(
                        (p.0 as i32, p.1 as i32, p.2),
                        (tip.0 as i32, tip.1 as i32, tip.2),
                        '+',
                        theme.mid,
                        theme.low,
                        true,
                    );
                }
            }
            Mode::Solid => {}
        }

        if glitch {
            for y in 0..buf.h {
                if y % 2 == 0 {
                    for x in 0..buf.w {
                        let idx = y * buf.w + x;
                        buf.fg[idx] = (
                            (buf.fg[idx].0 as u16 * 3 / 4) as u8,
                            (buf.fg[idx].1 as u16 * 3 / 4) as u8,
                            (buf.fg[idx].2 as u16 * 3 / 4) as u8,
                        );
                    }
                }
                if (rng.next() % 45) == 0 {
                    let shift = (rng.next() % 5) as usize + 1;
                    let row = y * buf.w;
                    buf.ch[row..row + buf.w].rotate_right(shift.min(buf.w));
                    buf.fg[row..row + buf.w].rotate_right(shift.min(buf.w));
                }
            }
        }

        if show_hud && buf.h > 4 {
            let top = format!(
                " TERM3D // [1-9/Tab] Mesh: {} ({}/{}) | [M] Mode: {} | [C] Theme: {} | {} FPS ",
                mesh.name,
                mesh_idx + 1,
                meshes.len(),
                mode.label(),
                theme.name,
                fps_display
            );
            let bot = format!(
                " [Mouse/WASD] Rotate | [+/-] Zoom:{:.1} | [X] Cull:{} | [L] OrbitLight:{} | [P] Pulse:{} | [G] CRT:{} | [B] Stars | [H] HUD | [Q] Quit ",
                dist,
                if cull { "ON" } else { "OFF" },
                if orbit_light { "ON" } else { "OFF" },
                if pulse { "ON" } else { "OFF" },
                if glitch { "ON" } else { "OFF" }
            );
            buf.text(1, 0, &top, theme.spec);
            buf.text(1, buf.h - 1, &bot, theme.high);
        }

        frame_str.clear();
        let _ = write!(frame_str, "\x1b[H");
        let mut last_col = (255u8, 255u8, 255u8);
        let mut first = true;

        for y in 0..buf.h {
            if y > 0 {
                frame_str.push_str("\r\n");
            }
            let row = y * buf.w;
            for x in 0..buf.w {
                let idx = row + x;
                let c = buf.ch[idx];
                let col = if c == ' ' { (0, 0, 0) } else { buf.fg[idx] };
                if first || col != last_col {
                    let _ = write!(frame_str, "\x1b[38;2;{};{};{}m", col.0, col.1, col.2);
                    last_col = col;
                    first = false;
                }
                frame_str.push(c);
            }
        }
        frame_str.push_str("\x1b[0m");

        let _ = execute!(out, MoveTo(0, 0));
        out.write_all(frame_str.as_bytes())?;
        out.flush()?;

        let frame_time = now.elapsed();
        let target = Duration::from_millis(16);
        if frame_time < target {
            std::thread::sleep(target - frame_time);
        }
    }
}