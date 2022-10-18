use std::{collections::HashMap, num::NonZeroU32, slice};

use glow::*;
use nalgebra::Vector3;

use crate::core::{material::Material, shader::Shader3D};

const AMBIENT_FACTOR: f32 = 0.5;

pub struct MeshModel<'a> {
    vertex_arrays: HashMap<String, Vec<f32>>,
    mesh_materials: HashMap<String, String>,
    materials: HashMap<String, Material>,
    vertex_counts: HashMap<String, i32>,
    vertex_buffer_ids: HashMap<String, NativeBuffer>,
    gl: &'a Context
}

impl<'a> MeshModel<'a> {
    pub fn new(gl: &'a Context) -> MeshModel<'a> {
        MeshModel {
            vertex_arrays: HashMap::new(),
            mesh_materials: HashMap::new(),
            materials: HashMap::new(),
            vertex_counts: HashMap::new(),
            vertex_buffer_ids: HashMap::new(),
            gl,
        }
    }

    pub fn add_vertex(&mut self, mesh_id: &str, position: Vector3<f32>, normal: Vector3<f32>) {
        if !self.vertex_arrays.contains_key(mesh_id) {
            self.vertex_arrays.insert(mesh_id.to_string(), Vec::new());
            self.vertex_counts.insert(mesh_id.to_string(), 0);
        }

        self.vertex_arrays.get_mut(mesh_id).unwrap().extend(vec![position.x, position.y, position.z, normal.x, normal.y, normal.z]);
        *self.vertex_counts.get_mut(mesh_id).unwrap() += 1;
    }

    pub fn set_mesh_material(&mut self, mesh_id: &str, mat_id: &str) {
        self.mesh_materials.insert(mesh_id.to_string(), mat_id.to_string());
    }

    pub fn add_material(&mut self, mat_id: &str, mat: &Material) {
        self.materials.insert(mat_id.to_string(), mat.clone());
    }

    pub fn set_opengl_buffers(&mut self) {
        for mesh_id in self.mesh_materials.keys() {
            unsafe {
                let buffer = self.gl.create_buffer().unwrap();
                self.vertex_buffer_ids.insert(mesh_id.to_string(), buffer);
                self.gl.bind_buffer(ARRAY_BUFFER, Some(self.vertex_buffer_ids[mesh_id]));
                self.gl.buffer_data_u8_slice(ARRAY_BUFFER,  slice::from_raw_parts(self.vertex_arrays[mesh_id].as_ptr() as *const u8, self.vertex_arrays[mesh_id].len() * 4), STATIC_DRAW);
                self.gl.bind_buffer(ARRAY_BUFFER, Some(NativeBuffer(NonZeroU32::new_unchecked(0))));
            }
        }
    }

    pub fn draw(&self, shader: &Shader3D) {
        for (mesh_id, mesh_material) in &self.mesh_materials {
            let material = self.materials[mesh_material];
            shader.set_material_diffuse(&material.diffuse);
            shader.set_material_specular(&material.specular);
            shader.set_material_ambient(AMBIENT_FACTOR);
            shader.set_shininess(material.shininess);
            shader.set_attribute_buffers(&self.vertex_buffer_ids[mesh_id]);

            unsafe {
                self.gl.draw_arrays(TRIANGLES, 0, self.vertex_counts[mesh_id]);
                self.gl.bind_buffer(ARRAY_BUFFER, Some(NativeBuffer(NonZeroU32::new_unchecked(0))));
            }
        }
    }
}
