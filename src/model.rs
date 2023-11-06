use crate::mesh::{Mesh, Texture, Vertex};
use crate::shader::Shader;
use crate::utils::{load_texture, to_c_str};
use russimp::material::{PropertyTypeInfo, TextureType};
use russimp::node::Node;
use russimp::scene::{PostProcess, PostProcessSteps, Scene};
use russimp::sys::aiMaterial;
use std::ops::Index;
use std::rc::Rc;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub directory: &'static str,
    textures_loaded: Vec<Texture>,
}

impl Model {
    pub fn new(path: &'static str) -> Self {
        let mut model = Self {
            meshes: vec![],
            directory: "",
            textures_loaded: vec![],
        };
        model.load_model(path);
        model
    }
    pub fn draw(&self, shader: &Shader) {
        for mesh in self.meshes.iter() {
            mesh.draw(shader);
        }
    }
    fn load_model(&mut self, path: &'static str) {
        println!("LOADING MODEL");
        match Scene::from_file(path, vec![PostProcess::Triangulate, PostProcess::FlipUVs]) {
            Err(error) => {
                println!("ERROR::ASSIMP::{:?}", error);
            }
            Ok(scene) => {
                let last_sep = path.rfind("/").unwrap();
                self.directory = &path[0..last_sep];
                if let Some(ref node) = scene.root {
                    self.process_node(node.clone(), &scene);
                }
                println!("FINISHED PROCESSING NODES");
            }
        }
    }
    fn process_node(&mut self, node: Rc<Node>, scene: &Scene) {
        println!("PROCESSING NODE {}", node.name);
        for mesh in node.meshes.iter() {
            let m = &scene.meshes[*mesh as usize];
            let mesh = self.process_mesh(m, scene);
            self.meshes.push(mesh);
        }

        let children = node.children.borrow();
        for child in children.iter() {
            self.process_node(child.clone(), scene);
        }
    }
    fn process_mesh(&mut self, mesh: &russimp::mesh::Mesh, scene: &Scene) -> Mesh {
        println!("PROCESSING MESH");
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut textures: Vec<Texture> = vec![];

        for i in 0..mesh.vertices.len() {
            let tex_coords = if let Some(ref tex_coord) = mesh.texture_coords[0] {
                glm::vec2(tex_coord[0].x, tex_coord[0].y)
            } else {
                glm::vec2(0.0, 0.0)
            };

            let vertex = Vertex {
                position: glm::vec3(mesh.vertices[i].x, mesh.vertices[i].y, mesh.vertices[i].z),
                normal: glm::vec3(mesh.normals[i].x, mesh.normals[i].y, mesh.normals[i].z),
                tex_coords,
            };
            vertices.push(vertex);
        }

        for face in mesh.faces.iter() {
            for index in face.0.iter() {
                indices.push(*index);
            }
        }

        if mesh.material_index >= 0 {
            let material = &scene.materials[mesh.material_index as usize];

            let mut diffuse_maps =
                self.load_material_textures(material, TextureType::Diffuse, "texture_diffuse");
            let mut specular_maps =
                self.load_material_textures(material, TextureType::Specular, "texture_specular");
            textures.append(&mut diffuse_maps);
            textures.append(&mut specular_maps);
        }
        return Mesh::new(vertices, indices, textures);
    }

    fn load_material_textures(
        &mut self,
        material: &russimp::material::Material,
        tex_type: russimp::material::TextureType,
        tex_name: &'static str,
    ) -> Vec<Texture> {
        let mut textures: Vec<Texture> = vec![];
        for property in material.properties.iter() {
            if property.semantic == tex_type {
                match property.data {
                    PropertyTypeInfo::String(ref str) => {
                        let file = format!("{}/{}", self.directory, str);
                        let mut skip = false;

                        println!("LOADED {:?}", self.textures_loaded);
                        for loaded_textures in self.textures_loaded.iter() {
                            if file == loaded_textures.path {
                                textures.push(loaded_textures.clone());
                                skip = true;
                                println!("SKIPPING");
                                break;
                            }
                        }

                        if !skip {
                            println!("NOT SKIPPING");
                            let texture = Texture {
                                id: load_texture(&file),
                                tex_type: tex_name,
                                path: file.clone(),
                            };
                            self.textures_loaded.push(texture.clone());
                            textures.push(texture.clone());
                        }
                    }
                    _ => {}
                };
            }
        }
        textures
    }
}
