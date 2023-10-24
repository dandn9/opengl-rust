use nalgebra_glm as glm;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}
// Default values
const YAW: f32 = -90.;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    // Camera attribtues
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,
    // Euler angles
    pub yaw: f32,
    pub pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(
        position: Option<glm::Vec3>,
        up: Option<glm::Vec3>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    ) -> Self {
        let mut camera = Self {
            position: position.unwrap_or(glm::Vec3::new(0., 0., 5.)),
            world_up: up.unwrap_or(glm::Vec3::new(0., 1., 0.)),
            front: glm::Vec3::new(0.0, 0.0, -1.0),
            right: glm::Vec3::default(),
            up: glm::Vec3::default(),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        };
        Camera::update_camera_vectors(&mut camera);
        return camera;
    }
    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn update_camera_vectors(&mut self) {
        self.front.x = f32::cos(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
        self.front.y = f32::sin(f32::to_radians(self.pitch));
        self.front.z = f32::sin(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
        self.front = glm::normalize(&self.front);
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        match direction {
            CameraMovement::FORWARD => self.position += self.front * velocity,
            CameraMovement::BACKWARD => self.position -= self.front * velocity,
            CameraMovement::LEFT => self.position -= self.right * velocity,
            CameraMovement::RIGHT => self.position += self.right * velocity,
        };
    }
    pub fn process_mouse_movement(
        &mut self,
        mut x_offset: f32,
        mut y_offset: f32,
        constrain_pitch: Option<bool>,
    ) {
        x_offset *= self.mouse_sensitivity;
        y_offset *= self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if constrain_pitch.unwrap_or(true) {
            if self.pitch > 89.0 {
                self.pitch = 89.0
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0
            }
        }
        self.update_camera_vectors();
    }
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        self.zoom -= yoffset;
        if self.zoom < 1.0 {
            self.zoom = 1.0
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0
        }
    }
}
