use crate::offset_of;
use crate::shader::Shader;
use crate::utils::ToCVoid;
use gl::types::*;
use std::ffi::c_void;
use std::mem::{size_of, size_of_val};

#[repr(C)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Texture {
    pub id: GLuint,
    pub tex_type: &'static str,
    pub path: String,
}

#[repr(C)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            vbo: 0,
            vao: 0,
            ebo: 0,
            textures: vec![],
            indices: vec![],
            vertices: vec![],
        }
    }
}
impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut mesh = Self {
            vertices,
            indices,
            textures,
            ..Mesh::default()
        };
        mesh.setup_mesh();
        mesh
    }
    pub fn draw(&self, shader: &Shader) {
        let mut diffuse_nr: GLuint = 1;
        let mut specular_nr: GLuint = 1;
        for (i, texture) in self.textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            }
            let mut number = String::new();
            let name = texture.tex_type;

            if name == "texture_diffuse" {
                number = diffuse_nr.to_string();
                diffuse_nr += 1;
            } else if name == "texture_specular" {
                number = specular_nr.to_string();
                specular_nr += 1;
            };

            shader.set_int(&format!("material.{}", name), i as i32);
            unsafe { gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id) }
        }
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                ToCVoid(0).into(),
            );
            gl::BindVertexArray(0);
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }
    fn setup_mesh(&mut self) {
        println!("SETTING UP MESH");
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
            let data = self.vertices.as_ptr() as *const c_void;

            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            let size = (self.indices.len() * size_of::<u32>()) as isize;
            let data = self.indices.as_ptr() as *const c_void;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let size = size_of::<Vertex>() as i32;
            // Vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size,
                offset_of!(Vertex, position) as *const c_void,
            );

            // Is there a way of coupling the type with the struct?
            let vertex_pos_offset = size_of::<glm::Vec3>();
            let vertex_normal_offset = size_of::<glm::Vec3>();
            // Vertex normal
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size,
                offset_of!(Vertex, normal) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                size,
                offset_of!(Vertex, tex_coords) as *const c_void,
            );
            gl::BindVertexArray(0);
        }
    }
}
