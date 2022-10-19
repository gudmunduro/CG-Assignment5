use std::borrow::Borrow;
use std::io::BufRead;
use std::{fs, io};

use crate::core::color::Color;
use crate::core::material::Material;
use crate::objects::mesh_model::MeshModel;

use glow::*;
use nalgebra::Vector3;

fn calculate_normals(positions: &Vec<Vector3<f32>>) -> Vec<Vector3<f32>> {
    let mut last = positions.last().unwrap();
    let mut next = &positions[1];

    let mut normals = Vec::new();
    for i in 0..positions.len() {
        let current = &positions[i];
        let v1 = last - current;
        let v2 = next - current;
        let v3 = v1.cross(&v2);

        normals.push(v3.normalize());
        last = current;
        next = if i+2 >= positions.len() {
            &positions[0]
        } else {
            &positions[i+2]
        };
    }

    normals
}

pub fn load_mtl_file(
    file_location: &str,
    file_name: &str,
    mesh_model: &mut MeshModel,
    gl: &Context,
) -> anyhow::Result<()> {
    println!("Start loading MTL: {file_name}");
    let mut mtl = None;
    let mut mtl_name: Option<String> = None;
    let fin = fs::File::open(format!("{file_location}/{file_name}"))?;
    let lines = io::BufReader::new(fin).lines();

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(e) => continue,
        };
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens.len() == 0 {
            continue;
        }

        match tokens[0] {
            "newmtl" => {
                if let Some(mtl) = &mtl {
                    mesh_model.add_material(&mtl_name.unwrap(), &mtl);
                }

                println!("Material {}", tokens[1]);
                mtl = Some(Material::new(None, None, None));
                mtl_name = Some(tokens[1].to_string());
            }
            "Kd" => {
                mtl.as_mut().unwrap().diffuse =
                    Color::new(tokens[1].parse()?, tokens[2].parse()?, tokens[3].parse()?);
            }
            "Ks" => {
                mtl.as_mut().unwrap().specular =
                    Color::new(tokens[1].parse()?, tokens[2].parse()?, tokens[3].parse()?);
            }
            "Ns" => {
                mtl.as_mut().unwrap().shininess = tokens[1].parse()?;
            }
            _ => (),
        }
    }
    println!("Finished loadaing MTL {file_name}");

    mesh_model.add_material(&mtl_name.unwrap(), &mtl.unwrap());

    Ok(())
}

pub fn load_obj_file<'a>(file_location: &str, file_name: &str, gl: &'a Context) -> anyhow::Result<MeshModel<'a>> {
    println!("Start loading OBJ: {file_name}");

    let mut mesh_model = MeshModel::new(gl);
    let mut current_object_id = None;
    let mut current_position_list = Vec::new();
    let mut current_normal_list = Vec::new();

    let fin = fs::File::open(format!("{file_location}/{file_name}"))?;
    let lines = io::BufReader::new(fin).lines();

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens.len() == 0 {
            continue;
        }

        match tokens[0] {
            "mtllib" => {
                load_mtl_file(file_location, tokens[1], &mut mesh_model, gl)?;
            }
            "o" => {
                println!("Mesh: {}", tokens[1]);
                current_object_id = Some(tokens[1].to_string());
            }
            "v" => {
                current_position_list.push(Vector3::new(
                    tokens[1].parse()?,
                    tokens[2].parse()?,
                    tokens[3].parse()?,
                ));
            }
            "vn" => {
                current_normal_list.push(Vector3::new(
                    tokens[1].parse()?,
                    tokens[2].parse()?,
                    tokens[3].parse()?,
                ));
            }
            "usemtl" => {
                mesh_model.set_mesh_material(current_object_id.as_ref().unwrap(), tokens[1]);
            }
            "f" => {
                let tokens = tokens
                    .iter()
                    .map(|&t| t.split("/").map(|s| s.to_string()).collect::<Vec<String>>())
                    .collect::<Vec<Vec<String>>>();
                
                //if current_normal_list.len() == 0 {
                //    current_normal_list = calculate_normals(&current_position_list);
                //}

                let vertex_count = tokens.len() - 1;
                for i in 0..(vertex_count -2) {
                    mesh_model.add_vertex(current_object_id.as_ref().unwrap(), current_position_list[(tokens[1][0].parse::<usize>()?)-1], current_normal_list[(tokens[1][2]).parse::<usize>()?-1]);
                    mesh_model.add_vertex(current_object_id.as_ref().unwrap(), current_position_list[(tokens[i+2][0].parse::<usize>()?)-1], current_normal_list[(tokens[i+2][2]).parse::<usize>()?-1]);
                    mesh_model.add_vertex(current_object_id.as_ref().unwrap(), current_position_list[(tokens[i+3][0].parse::<usize>()?)-1], current_normal_list[(tokens[i+3][2]).parse::<usize>()?-1]);
                }
            }
            _ => (),
        }
    }

    mesh_model.set_opengl_buffers();

    Ok(mesh_model)
}
