//#![no_implicit_prelude]
#![allow(dead_code)]
// extern crate alloc;
// use alloc::vec::Vec;
#[derive(Debug, Copy, Clone)]
pub struct Vert {
    coords: [f32; 4],
}
pub struct Plane {
    coord: Vert,
    norm: Vert,
}

pub struct M3 {
    pub col1: Vert,
    pub col2: Vert,
    pub col3: Vert,
}

pub struct M4 {
    col1: Vert,
    col2: Vert,
    col3: Vert,
    col4: Vert,
}
pub struct Ray {
    point: Vert,
    vert: Vert,
}

pub struct TextureCoord {
    coords: Vert,
}

pub struct VertNorm {
    coords: Vert,
}

pub struct SpaceVert {
    coords: Vert,
}

pub struct Face3 {
    verts: M3,
    normals: Option<M3>,
    texture_coords: Option<M3>,
}

pub fn dot_v3v3(one: &[f32; 4], two: &[f32; 4]) -> f32 {
    one[0] * two[0] + one[1] * two[1] + one[2] + two[2]
}

pub fn cross_v3v3(one: &[f32; 4], two: &[f32; 4]) -> [f32; 4] {
    [
        one[1] * two[2] - one[2] * two[1],
        one[0] * two[2] - one[2] * two[0],
        one[0] * two[1] - one[1] * two[0],
        -1.0,
    ]
}

pub fn add_v3v3(one: &[f32; 4], two: &[f32; 4]) -> [f32; 4] {
    [one[0] + two[0], one[1] + two[1], one[2] + two[2], -1.0]
}

pub fn sub_v3v3(one: &[f32; 4], two: &[f32; 4]) -> [f32; 4] {
    [one[0] - two[0], one[1] - two[1], one[2] - two[2], -1.0]
}

pub fn mul_v3fl(v: &[f32; 4], f: &f32) -> [f32; 4] {
    [v[0] * f, v[1] * f, v[2] * f, -1.0]
}

pub fn points_to_plane(one: &Vert, two: &Vert, three: &Vert) -> Plane {
    let coord = Vert {
        coords: cross_v3v3(
            &sub_v3v3(&two.coords, &one.coords),
            &sub_v3v3(&three.coords, &one.coords),
        ),
    };
    let norm = one.clone();
    Plane {
        coord: coord,
        norm: norm,
    }
}

pub fn ray_isect_plane(r: &Ray, pl: &Plane) -> Vert {
    let dot = dot_v3v3(&pl.norm.coords, &r.vert.coords);
    let w = sub_v3v3(&r.point.coords, &pl.coord.coords);
    let dot2 = dot_v3v3(&pl.norm.coords, &w);
    let fac = -1.0 * dot2 / dot;
    let u = mul_v3fl(&r.vert.coords, &fac);
    Vert {
        coords: add_v3v3(&r.point.coords, &u),
    }
}

pub fn dist_v3v3(a: [f32; 4], b: [f32; 4]) -> f32 {
    ((a[0] - b[0]).powf(2.0) + (a[1] - b[1]).powf(2.0) + (a[2] - b[2]).powf(2.0)).sqrt()
}

pub fn p_inside_tri(p: &Vert, v1: &Vert, v2: &Vert, v3: &Vert) -> bool {
    let d1 = dist_v3v3(p.coords, v1.coords);
    let d2 = dist_v3v3(p.coords, v2.coords);
    let d3 = dist_v3v3(p.coords, v3.coords);
    let total = d1 + d2 + d3;
    d1 > 0.0 && d2 > 0.0 && d3 > 0.0 && d1 < total && d2 < total && d3 < total
}

pub fn m3m(one: M3, two: M3) -> M3 {
    let col1 = Vert {
        coords: [
            one.col1.coords[0] * two.col1.coords[0]
                + one.col2.coords[0] * two.col1.coords[1]
                + one.col3.coords[0] * two.col1.coords[2],
            one.col1.coords[1] * two.col1.coords[0]
                + one.col2.coords[1] * two.col1.coords[1]
                + one.col3.coords[1] * two.col1.coords[2],
            one.col1.coords[2] * two.col1.coords[0]
                + one.col2.coords[2] * two.col1.coords[1]
                + one.col3.coords[2] * two.col1.coords[2],
            -1.0,
        ],
    };
    let col2 = Vert {
        coords: [
            one.col1.coords[0] * two.col2.coords[0]
                + one.col2.coords[0] * two.col2.coords[1]
                + one.col3.coords[0] * two.col2.coords[2],
            one.col1.coords[1] * two.col2.coords[0]
                + one.col2.coords[1] * two.col2.coords[1]
                + one.col3.coords[1] * two.col2.coords[2],
            one.col1.coords[2] * two.col2.coords[0]
                + one.col2.coords[2] * two.col2.coords[1]
                + one.col3.coords[2] * two.col2.coords[2],
            -1.0,
        ],
    };
    let col3 = Vert {
        coords: [
            one.col1.coords[0] * two.col3.coords[0]
                + one.col2.coords[0] * two.col3.coords[1]
                + one.col3.coords[0] * two.col3.coords[2],
            one.col1.coords[1] * two.col3.coords[0]
                + one.col2.coords[1] * two.col3.coords[1]
                + one.col3.coords[1] * two.col3.coords[2],
            one.col1.coords[2] * two.col3.coords[0]
                + one.col2.coords[2] * two.col3.coords[1]
                + one.col3.coords[2] * two.col3.coords[2],
            -1.0,
        ],
    };
    M3 {
        col1: col1,
        col2: col2,
        col3: col3,
    }
}

pub fn m4m(one: M4, two: M4) -> M4 {
    let col1 = Vert {
        coords: [
            one.col1.coords[0]
                + two.col1.coords[0]
                + one.col2.coords[0] * two.col1.coords[1]
                + one.col3.coords[0] * two.col1.coords[2]
                + one.col4.coords[0] * two.col1.coords[3],
            one.col1.coords[1]
                + two.col1.coords[0]
                + one.col2.coords[1] * two.col1.coords[1]
                + one.col3.coords[1] * two.col1.coords[2]
                + one.col4.coords[1] * two.col1.coords[3],
            one.col1.coords[2]
                + two.col1.coords[0]
                + one.col2.coords[2] * two.col1.coords[1]
                + one.col3.coords[2] * two.col1.coords[2]
                + one.col4.coords[2] * two.col1.coords[3],
            one.col1.coords[3]
                + two.col1.coords[0]
                + one.col2.coords[3] * two.col1.coords[1]
                + one.col3.coords[3] * two.col1.coords[2]
                + one.col4.coords[3] * two.col1.coords[3],
        ],
    };
    let col2 = Vert {
        coords: [
            one.col1.coords[0]
                + two.col2.coords[0]
                + one.col2.coords[0] * two.col2.coords[1]
                + one.col3.coords[0] * two.col2.coords[2]
                + one.col4.coords[0] * two.col2.coords[3],
            one.col1.coords[1]
                + two.col2.coords[0]
                + one.col2.coords[1] * two.col2.coords[1]
                + one.col3.coords[1] * two.col2.coords[2]
                + one.col4.coords[1] * two.col2.coords[3],
            one.col1.coords[2]
                + two.col2.coords[0]
                + one.col2.coords[2] * two.col2.coords[1]
                + one.col3.coords[2] * two.col2.coords[2]
                + one.col4.coords[2] * two.col2.coords[3],
            one.col1.coords[3]
                + two.col2.coords[0]
                + one.col2.coords[3] * two.col2.coords[1]
                + one.col3.coords[3] * two.col2.coords[2]
                + one.col4.coords[3] * two.col2.coords[3],
        ],
    };
    let col3 = Vert {
        coords: [
            one.col1.coords[0]
                + two.col3.coords[0]
                + one.col2.coords[0] * two.col3.coords[1]
                + one.col3.coords[0] * two.col3.coords[2]
                + one.col4.coords[0] * two.col3.coords[3],
            one.col1.coords[1]
                + two.col3.coords[0]
                + one.col2.coords[1] * two.col3.coords[1]
                + one.col3.coords[1] * two.col3.coords[2]
                + one.col4.coords[1] * two.col3.coords[3],
            one.col1.coords[2]
                + two.col3.coords[0]
                + one.col2.coords[2] * two.col3.coords[1]
                + one.col3.coords[2] * two.col3.coords[2]
                + one.col4.coords[2] * two.col3.coords[3],
            one.col1.coords[3]
                + two.col3.coords[0]
                + one.col2.coords[3] * two.col3.coords[1]
                + one.col3.coords[3] * two.col3.coords[2]
                + one.col4.coords[3] * two.col3.coords[3],
        ],
    };
    let col4 = Vert {
        coords: [
            one.col1.coords[0]
                + two.col4.coords[0]
                + one.col2.coords[0] * two.col4.coords[1]
                + one.col3.coords[0] * two.col4.coords[2]
                + one.col4.coords[0] * two.col4.coords[3],
            one.col1.coords[1]
                + two.col4.coords[0]
                + one.col2.coords[1] * two.col4.coords[1]
                + one.col3.coords[1] * two.col4.coords[2]
                + one.col4.coords[1] * two.col4.coords[3],
            one.col1.coords[2]
                + two.col4.coords[0]
                + one.col2.coords[2] * two.col4.coords[1]
                + one.col3.coords[2] * two.col4.coords[2]
                + one.col4.coords[2] * two.col4.coords[3],
            one.col1.coords[3]
                + two.col4.coords[0]
                + one.col2.coords[3] * two.col4.coords[1]
                + one.col3.coords[3] * two.col4.coords[2]
                + one.col4.coords[3] * two.col4.coords[3],
        ],
    };
    M4 {
        col1: col1,
        col2: col2,
        col3: col3,
        col4: col4,
    }
}

fn main() {
    println!("done!");
}
