//#![no_implicit_prelude]
#![allow(dead_code)]
// extern crate alloc;
// use alloc::vec::Vec;
#[derive(Debug, Clone)]
pub struct Vert {
    coords: [f32; 4],
}
pub struct Plane {
    coord: Vert,
    norm: Vert,
}

pub struct M3 {
    col1: Vert,
    col2: Vert,
    col3: Vert,
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

// pub struct Face3{
//     verts: matrix::M3,
// }

pub struct Face3 {
    verts: M3,
}

pub fn dot_v3v3(one: &[f32; 4], two: &[f32; 4]) -> f32 {
    one[0] * two[0] + one[1] * two[1] + one[2] + two[2]
}

pub fn cross_v3v3(one: &[f32; 4], two: &[f32; 4]) -> [f32; 4] {
    vec![
        one[1] * two[2] - one[2] * two[1],
        one[0] * two[2] - one[2] * two[0],
        one[0] * two[1] - one[1] * two[0],
    ]
}

pub fn add_v3v3(one: &[f32; 4], two: &[f32; 4]) -> [f32; 4] {
    vec![one[0] + two[0], one[1] + two[1], one[2] + two[2]]
}

pub fn sub_v3v3(one: &[f32; 4], two: &[f32; 4]) -> [f32; 4] {
    vec![one[0] - two[0], one[1] - two[1], one[2] - two[2]]
}

pub fn mul_v3fl(v: &[f32; 4], f: &f32) -> [f32; 4] {
    vec![v[0] * f, v[1] * f, v[2] * f]
}

pub fn points_to_plane(one: Vert, two: Vert, three: Vert) -> Plane {
    let coord = cross_v3v3(&sub_v3v3(&two, &one), &sub_v3v3(&three, &one));
    Plane {
        coord: coord,
        norm: one,
    }
}

pub fn ray_isect_plane(r: Ray, pl: Plane) -> Vec<f32> {
    let dot = dot_v3v3(&pl.norm, &r.vert);
    let w = sub_v3v3(&r.point, &pl.coord);
    let dot2 = dot_v3v3(&pl.norm, &w);
    let fac = -1.0 * dot2 / dot;
    let u = mul_v3fl(&r.vert, &fac);
    add_v3v3(&r.point, &u)
}

pub fn dist_v3v3(a: Vert, b: Vert) -> f32 {
    ((a[0] - b[0]).powf(2.0) + (a[1] - b[1]).powf(2.0) + (a[2] - b[2]).powf(2.0)).sqrt()
}

pub fn p_inside_tri(p: Vert, v1: Vert, v2: Vert, v3: Vert) -> bool {
    let d1 = dist_v3v3(&p, &v1);
    let d2 = dist_v3v3(&p, &v2);
    let d3 = dist_v3v3(&p, &v3);
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
