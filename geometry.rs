#![allow(dead_code)]
#[derive(Copy, Clone)]
mod matrix;
use std:ptr;

pub struct Vert3{
    coords: matrix::Vert3,
}

pub struct TextureCoord{
    coords: matrix::Vert3,
}

pub struct VertNorm{
    coords: matrix::Vert3,
}

pub struct SpaceVert{
    coords: matrix::Vert3,
}

pub struct Face3{
    verts: matrix::M3,
}

pub struct PlusThreeFace{
    verts: Vec<Vert3>,
}

pub fn process_vert3(s: String) -> Vert{
    let verts = s.split(" ");
    Vert3{
        x: verts[1].parse::<f32>,
        y: verts[2].parse::<f32>,
        z: verts[3].parse::<f32>,
    }
}

pub fn process_line(
    s: String,
    verts: Vec<Vert>,
    texture_coords: Vec<TextureCoord>,
    vert_norms: Vec<VertNorm>,
    faces: Vec<Face3>){

    let parts = s.split(" ");
    let delimeter = parts[0].parse::<i32>;
    if delimeter == "v"{
        verts.push(Vert3{
            coords.x: parts[1].parse::<f32>,
            coords.y: parts[2].parse::<f32>,
            coords.z: parts[3].parse::<f32>,
        });
    }else if delimeter == "vt"{
        if parts.length() == 1{
            texture_coords.push(TextureCoord{
                coords.x: parts[1].parse::<f32>,
                ..Default::default() }
            );
        } else if parts.lengt() == 2{
            texture_coords.push(TextureCoord{
                coords.x: parts[1].parse::<f32>,
                coords.y: parts[2].parse::<f32>,
                ..Default::default() }
            );
        } else{
            texture_coords.push(TextureCoord{
                coords.x: parts[1].parse::<f32>,
                coords.y: parts[2].parse::<f32>,
                coords.z: parts[3].parse::<f32>,}
            );
        
            
                             

}       

fn main(){
    println!("geometry good!");
}
