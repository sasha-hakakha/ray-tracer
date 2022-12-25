#![allow(dead_code)]

// extern crate alloc;
// use alloc::{vec::Vec, string::String};
// extern crate core;
// use core::option::Option::{self,*};
// use core::iter::Iterator;
// use core::panic::Panic;

mod matrix;



// pub fn _many_points_to_tri(verts:&[matrix::Vert3], points: Vec<&str>) -> Vec<matrix::Face3M> {
//     let mut faces = Vec::new();
//     for i in 0..points.len() - 2 {
//         faces.push(matrix::Face3M{
//             col1:*verts[points[0].parse::<usize>().unwrap()], 
//             col2:*verts[points[i].parse::<usize>().unwrap()], 
//             col3:*verts[points[i + 1].parse::<usize>().unwrap()]
//             }
//         );
//     }
//     faces
// }
pub fn many_points_to_tri(verts:&[matrix::Vert3], points: &[&str]) -> Vec<matrix::Face3M> {
    let mut faces = Vec::new();
    for i in 0..points.len() - 2 {
        faces.push(matrix::Face3M{
            col1:verts[points[0].parse::<usize>().unwrap()], 
            col2:verts[points[i].parse::<usize>().unwrap()],
            col3:verts[points[i + 1].parse::<usize>().unwrap()]
            }
        );
    }
    faces
}
// pub fn three_verts_to_face(points: Vec<String>, verts: &[Vert3]) -> Face {
//     Face3M { 
//         col1: verts[points[0].parse::<i32>().unwrap()],
//         col2: verts[points[1].parse::<i32>().unwrap()],
//         col3: verts[points[2].parse::<i32>().unwrap()]
//     }
// }

// pub fn three_verts_to_face(points: Vec<&str>, verts: &[matrix::Vert3]) -> matrix::Face3M {
//     matrix::Face3M { 
//         col1: verts[points[0].parse::<usize>().unwrap()],
//         col2: verts[points[1].parse::<usize>().unwrap()],
//         col3: verts[points[2].parse::<usize>().unwrap()]
//     }
// }
pub fn three_verts_to_face(points: Vec<&str>, verts: &Vec<matrix::Vert3>) -> matrix::Face3M {
    let col1_index = points[0].parse::<usize>().unwrap();
    let col2_index = points[1].parse::<usize>().unwrap();
    let col3_index = points[2].parse::<usize>().unwrap();

    matrix::Face3M {
        col1: verts[col1_index],
        col2: verts[col2_index],
        col3: verts[col3_index]
    }
}

pub fn process_line(
    s: String,
    verts: &Vec<matrix::Vert3>,
    texture_coords: Vec<matrix::TextureCoord>,
    vert_norms: Vec<matrix::VertNorm>,
    faces: Vec<matrix::Face3M>){

    let parts = s.split(" ").collect::<Vec<&str>>();
    let delimeter = parts[0];

    if delimeter == "v"{
        verts.push(matrix::Vert3{
            coords:vec!{
                parts[1].parse::<f32>().unwrap(),
                parts[2].parse::<f32>().unwrap(),
                parts[3].parse::<f32>().unwrap(),
            }
        });
    }else if delimeter == "vt"{
        if parts.len() == 1{
            texture_coords.push(matrix::TextureCoord{
                coords:vec!{
                    parts[1].parse::<f32>().unwrap(),
                    -1.0, -1.0
                }
            });
        } else if parts.len() == 2{
            texture_coords.push(matrix::TextureCoord{
                coords:vec!{
                    parts[1].parse::<f32>().unwrap(),
                    parts[2].parse::<f32>().unwrap(),
                    -1.0,
                }
            });
        } else{
            texture_coords.push(matrix::TextureCoord{
                coords:vec!{
                    parts[1].parse::<f32>().unwrap(),
                    parts[2].parse::<f32>().unwrap(),
                    parts[3].parse::<f32>().unwrap(),
                }
            });
        }
    }
    else if delimeter == "f"{
        if parts.len() < 3 {panic!("Face has less than 3 points!")}
        else if parts.len() == 3{
            faces.push(three_verts_to_face(
                parts,
                verts,
            ));
        }
        else{
            let mut new =  many_points_to_tri(
                verts,
                &parts,
            );
            faces.append(&mut new);
        }
    }

} 
            
fn main(){
    println!("geometry good!");
}
