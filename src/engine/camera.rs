use super::matrices::{ perspective_matrix, view_matrix };

pub struct Camera{
    pub player_pos: [f32; 3],
    pub player_rot: [f32; 3],
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
    pub projection: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(initial_pos: [f32; 3], initral_rot: [f32; 3], move_speed: f32, mouse_sensitivity: f32,terminal_size: (u32, u32)) -> Camera {
        Camera {
            player_pos: initial_pos,
            player_rot: initral_rot,
            move_speed: move_speed,
            mouse_sensitivity: mouse_sensitivity,
            projection: perspective_matrix(terminal_size),
            view: view_matrix(&initial_pos, &initral_rot),
        }
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        view_matrix(&self.player_pos, &self.player_rot)
    }

    pub fn perspective_matrix(&self) -> [[f32; 4]; 4] {
        self.projection
    }

    pub fn update(&mut self, terminal_size: (u32,u32), move_vector: [i8; 3], mouse_vector: [i8; 2]) {
        self.view = view_matrix(&self.player_pos, &self.player_rot);
        self.projection = perspective_matrix(terminal_size);

        
        match move_vector[0] {
            -1 => self.move_left(),
            1 => self.move_right(),
            _ => (),
        }

        match move_vector[1] {
            1 => self.move_up(),
            -1 => self.move_down(),
            _ => (),
        }

        match move_vector[2] {
            1 => self.move_forward(),
            -1 => self.move_backward(),
            _ => (),
        }


        match mouse_vector[1] {
            1=> self.rotate_up(),
            -1 => self.rotate_down(),
            _ => (),
        }

        match mouse_vector[0] {
            -1=> self.rotate_left(),
            1 => self.rotate_right(),
            _ => (),
        }

    }

    fn move_forward(&mut self) {
        self.player_pos[0] += self.player_rot[1].sin() * self.move_speed;
        self.player_pos[2] += self.player_rot[1].cos() * self.move_speed;
    }

    fn move_backward(&mut self) {
        self.player_pos[0] -= self.player_rot[1].sin() * self.move_speed;
        self.player_pos[2] -= self.player_rot[1].cos() * self.move_speed;
    }

    fn move_left(&mut self) {
        self.player_pos[0] -= self.player_rot[1].cos() * self.move_speed;
        self.player_pos[2] += self.player_rot[1].sin() * self.move_speed;
    }

    fn move_right(&mut self) {
        self.player_pos[0] += self.player_rot[1].cos() * self.move_speed;
        self.player_pos[2] -= self.player_rot[1].sin() * self.move_speed;
    }

    fn move_up(&mut self) {
        self.player_pos[1] += self.move_speed;
    }

    fn move_down(&mut self) {
        self.player_pos[1] -= self.move_speed;
    }

    fn rotate_up(&mut self) {
        self.player_rot[0] -= self.mouse_sensitivity;
    }

    fn rotate_down(&mut self) {
        self.player_rot[0] += self.mouse_sensitivity;
    }

    fn rotate_left(&mut self) {
        self.player_rot[1] -= self.mouse_sensitivity;
    }

    fn rotate_right(&mut self) {
        self.player_rot[1] += self.mouse_sensitivity;
    } 
}