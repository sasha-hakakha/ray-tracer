#![allow(dead_code)]
#[derive(Copy, Clone)]

use ptr;

pub struct Vec3{
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl Default for Vec3{
    fn default() -> Vec3 {
        Vec3{
            x: None,
            y: None,
            z: None,
        }
    }
}

pub struct Vec4{
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
    w: Option<f32>,
}

pub struct V3Ray{
    point: Vec3,
    vert: Vec3,
}


pub struct Plane{
    coord: Vec3,
    norm: Vec3,
}

pub struct M3{
    col1: Vec3,
    col2: Vec3,
    col3: Vec3,
}

pub struct M4{
    col1: Vec4,
    col2: Vec4,
    col3: Vec4,
    col4: Vec4,
}

pub fn dot_v3v3(one: Vec3, two: Vec3) -> f32{
    one.x * two.x + one.y * two.y + one.z + two.z
}

pub fn cross_v3v3(one: Vec3, two: Vec3) -> Vec3 {
    Vec3{
        x: one.y * two.z - one.z * two.y,
        y: one.x * two.z - one.z * two.x,
        z: one.x * two.y - one.y * two.x,
    }
}

pub fn add_v3v3(one: Vec3, two: Vec3) -> Vec3 {
    Vec3{
        x: one.x + two.x,
        y: one.y + two.y,
        z: one.z + two.z,
    }
}

pub fn sub_v3v3(one: Vec3, two: Vec3) -> Vec3 {
    Vec3{
        x: one.x - two.x,
        y: one.y - two.y,
        z: one.z - two.z,
    }
}  

pub fn mul_v3fl(v: Vec3, f: f32) -> Vec3{
    Vec3{
        x: v.x * f,
        y: v.y * f,
        z: v.z * f,
    }
}

pub fn points_to_plane(one: Vec3, two: Vec3, three: Vec3) -> Plane{
    let coord = cross_v3v3(sub_v3v3(two, one), sub_v3v3(three, one));
    Plane{
        coord: coord,
        norm: one,
    }
}

pub fn ray_isect_plane(r: V3Ray, pl: Plane) -> Vec3{
    let dot = dot_v3v3(pl.norm, r.vert);
    let w = sub_v3v3(r.point, pl.coord);
    let dot2 = dot_v3v3(pl.norm, w);
    let fac = -1.0 * dot2 / dot;
    let u = mul_v3fl(r.vert, fac);
    add_v3v3(r.point, u)
}

pub fn dist_v3v3(a: Vec3, b: Vec3) -> f32{
   ((a.x-b.x).powf(2.0) + (a.y-b.y).powf(2.0) + (a.z-b.z).powf(2.0)).sqrt()
}

pub fn p_inside_tri(p: Vec3, v1: Vec3, v2: Vec3, v3: Vec3) -> bool{
    let d1 = dist_v3v3(p, v1);
    let d2 = dist_v3v3(p, v2);
    let d3 = dist_v3v3(p, v3);
    let total = d1 + d2 + d3;
    d1 > 0 && d2 > 0 && d3 > 0 && d1 < total && d2 < total && d3 < total
}

pub fn m3m(one: M3, two: M3){
    let col1 = Vec3{
        x: one.col1.x * two.col1.x + one.col2.x * two.col1.y + one.col3.x * two.col1.z,
        y: one.col1.y * two.col1.x + one.col2.y * two.col1.y + one.col3.y * two.col1.z,
        z: one.col1.z * two.col1.x + one.col2.z * two.col1.y + one.col3.z * two.col1.z,
    };
    let col2 = Vec3{
        x: one.col1.x * two.col2.x + one.col2.x * two.col2.y + one.col3.x * two.col2.z,
        y: one.col1.y * two.col2.x + one.col2.y * two.col2.y + one.col3.y * two.col2.z,
        z: one.col1.z * two.col2.x + one.col2.z * two.col2.y + one.col3.z * two.col2.z,
    };
    let col3 = Vec3{
        x: one.col1.x * two.col3.x + one.col2.x * two.col3.y + one.col3.x * two.col3.z,
        y: one.col1.y * two.col3.x + one.col2.y * two.col3.y + one.col3.y * two.col3.z,
        z: one.col1.z * two.col3.x + one.col2.z * two.col3.y + one.col3.z * two.col3.z,
    };
    M3{
        col1: col1,
        col2: col2,
        col3: col3,
    }   
}   

pub fn m4m(one: M4, two: M4){
    let col1 = Vec4{
        x: one.col1.x + two.col1.x + one.col2.x * two.col1.y + one.col3.x * two.col1.z + one.col4.x * two.col1.w,
        y: one.col1.y + two.col1.x + one.col2.y * two.col1.y + one.col3.y * two.col1.z + one.col4.y * two.col1.w,
        z: one.col1.z + two.col1.x + one.col2.z * two.col1.y + one.col3.z * two.col1.z + one.col4.z * two.col1.w,
        w: one.col1.w + two.col1.x + one.col2.w * two.col1.y + one.col3.w * two.col1.z + one.col4.w * two.col1.w,
    };
    let col2 = Vec4{
        x: one.col1.x + two.col2.x + one.col2.x * two.col2.y + one.col3.x * two.col2.z + one.col4.x * two.col2.w,
        y: one.col1.y + two.col2.x + one.col2.y * two.col2.y + one.col3.y * two.col2.z + one.col4.y * two.col2.w,
        z: one.col1.z + two.col2.x + one.col2.z * two.col2.y + one.col3.z * two.col2.z + one.col4.z * two.col2.w,
        w: one.col1.w + two.col2.x + one.col2.w * two.col2.y + one.col3.w * two.col2.z + one.col4.w * two.col2.w,
    };
    let col3 = Vec4{
        x: one.col1.x + two.col3.x + one.col2.x * two.col3.y + one.col3.x * two.col3.z + one.col4.x * two.col3.w,
        y: one.col1.y + two.col3.x + one.col2.y * two.col3.y + one.col3.y * two.col3.z + one.col4.y * two.col3.w,
        z: one.col1.z + two.col3.x + one.col2.z * two.col3.y + one.col3.z * two.col3.z + one.col4.z * two.col3.w,
        w: one.col1.w + two.col3.x + one.col2.w * two.col3.y + one.col3.w * two.col3.z + one.col4.w * two.col3.w,
    };
    let col2 = Vec4{
        x: one.col1.x + two.col4.x + one.col2.x * two.co4.y + one.col3.x * two.co4.z + one.col4.x * two.co4.w,
        y: one.col1.y + two.col4.x + one.col2.y * two.co4.y + one.col3.y * two.co4.z + one.col4.y * two.co4.w,
        z: one.col1.z + two.col4.x + one.col2.z * two.co4.y + one.col3.z * two.co4.z + one.col4.z * two.co4.w,
        w: one.col1.w + two.col4.x + one.col2.w * two.co4.y + one.col3.w * two.co4.z + one.col4.w * two.co4.w,
    };
    M4{
        col1: col1,
        col2: col2,
        col3: col3,
        col4: col4,
    }
}

fn main(){
    println!("done!");
}
