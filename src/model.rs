use crate::mesh::{Mesh, Texture, Vertex};
use crate::shader::Shader;
use crate::utils::{load_texture, to_c_str};
use russimp::material::{PropertyTypeInfo, TextureType};
use russimp::node::Node;
use russimp::scene::{PostProcess, PostProcessSteps, Scene};
use std::ops::Index;
use std::path::Path;
use std::rc::Rc;
use std::time::SystemTime;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub directory: &'static str,
    loaded_textures: Vec<Texture>,
}

impl Model {
    pub fn new(path: &'static str) -> Self {
        let mut model = Self {
            meshes: vec![],
            directory: "",
            loaded_textures: vec![],
        };
        model.load_model_obj(path);
        model
    }
    pub fn draw(&self, shader: &Shader) {
        for mesh in self.meshes.iter() {
            mesh.draw(shader);
        }
    }
    fn load_model_obj(&mut self, path: &'static str) {
        let last_sep = path.rfind("/").unwrap();
        self.directory = &path[0..last_sep];

        // Using tobj's default options the vertex normals will be not the same number of vertex positions :\
        let (models, materials) =
            tobj::load_obj(&Path::new(path), &tobj::GPU_LOAD_OPTIONS).unwrap();
        let materials = materials.unwrap();

        for model in models.iter() {
            let mesh = &model.mesh;
            assert_eq!(mesh.positions.len() % 3, 0);
            let vertices_count = mesh.positions.len() / 3;
            println!("MESH {}", vertices_count);
            let mut vertices: Vec<Vertex> = Vec::with_capacity(vertices_count);
            let mut indices: Vec<u32> = mesh.indices.clone();
            let mut textures: Vec<Texture> = vec![];

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            // Process vertices
            for i in 0..vertices_count {
                vertices.push(Vertex {
                    position: glm::vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: glm::vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    tex_coords: glm::vec2(t[i * 2], t[i * 2 + 1]),
                })
            }

            // Process textures
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];
                textures = self.load_material_textures_tobj(&material);
            }

            let mesh = Mesh::new(vertices, indices, textures);
            self.meshes.push(mesh);
            // for vertex in mesh.positions
        }
    }
    fn load_material_textures_tobj(&mut self, material: &tobj::Material) -> Vec<Texture> {
        let mut textures: Vec<Texture> = vec![];
        // Diffuse texture
        if let Some(ref diffuse_texture) = material.diffuse_texture {
            let path = format!("{}/{}", self.directory, diffuse_texture);
            let mut skip = false;
            for loaded_texture in self.loaded_textures.iter() {
                if &loaded_texture.path == &path {
                    textures.push(loaded_texture.clone());
                    skip = true;
                }
            }

            if !skip {
                let texture = Texture {
                    id: load_texture(&path),
                    path,
                    tex_type: "texture_diffuse",
                };
                self.loaded_textures.push(texture.clone());
                textures.push(texture);
            }
        }
        if let Some(ref specular_texture) = material.specular_texture {
            let path = format!("{}/{}", self.directory, specular_texture);
            let mut skip = false;
            for loaded_texture in self.loaded_textures.iter() {
                if &loaded_texture.path == &path {
                    textures.push(loaded_texture.clone());
                    skip = true;
                }
            }

            if !skip {
                let texture = Texture {
                    id: load_texture(&path),
                    path,
                    tex_type: "texture_specular",
                };
                self.loaded_textures.push(texture.clone());
                textures.push(texture);
            }
        }
        textures
    }

    fn load_model_russimp(&mut self, path: &'static str) {
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
        let start = SystemTime::now();
        println!("PROCESSING NODE {} {:?}", node.name, start);
        for mesh in node.meshes.iter() {
            let m = &scene.meshes[*mesh as usize];
            let mesh = self.process_mesh(m, scene);
            self.meshes.push(mesh);
        }
        println!("FINISHED - {:?}", start.elapsed());

        let children = node.children.borrow();
        for child in children.iter() {
            self.process_node(child.clone(), scene);
        }
    }
    fn process_mesh(&mut self, mesh: &russimp::mesh::Mesh, scene: &Scene) -> Mesh {
        let start = SystemTime::now();
        println!("PROCESSING MESH {:?}", start);
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

        println!("PROCESSING MESH {:?}", start.elapsed());
        if mesh.material_index >= 0 {
            let material = &scene.materials[mesh.material_index as usize];

            let mut diffuse_maps =
                self.load_material_textures(material, TextureType::Diffuse, "texture_diffuse");
            let mut specular_maps =
                self.load_material_textures(material, TextureType::Specular, "texture_specular");
            textures.append(&mut diffuse_maps);
            textures.append(&mut specular_maps);
        }
        println!("FINISHED PROCESSING MESH {:?}", start.elapsed());
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

                        println!("LOADED {:?}", self.loaded_textures);
                        for loaded_textures in self.loaded_textures.iter() {
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
                            self.loaded_textures.push(texture.clone());
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
