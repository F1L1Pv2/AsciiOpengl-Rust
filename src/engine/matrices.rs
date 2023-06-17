fn rotate_x(angle: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, angle.cos(), -angle.sin(), 0.0],
        [0.0, angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn rotate_y(angle: f32) -> [[f32; 4]; 4] {
    [
        [angle.cos(), 0.0, angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn rotate_z(angle: f32) -> [[f32; 4]; 4] {
    [
        [angle.cos(), -angle.sin(), 0.0, 0.0],
        [angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0f32, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn scale(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn translate(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0f32, 0.0],
        [x, y, z, 1.0f32],
    ]
}

//create macro for matrix multiplication
macro_rules! mat_mul {
    ($a:expr, $b:expr) => {{
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += $a[i][k] * $b[k][j];
                }
            }
        }
        result
    }};
}

// let perspective = {
//     // let (width, height) = target.get_dimensions();
//     let (width, height) = terminal_size;
//     let aspect_ratio = (height as f32) / (width as f32);

//     let fov: f32 = 3.141592 / 3.0;
//     let zfar = 1024.0;
//     let znear = 0.1;

//     let f = 1.0 / (fov / 2.0).tan();

//     [
//         [f * aspect_ratio, 0.0, 0.0, 0.0],
//         [0.0, f, 0.0, 0.0],
//         [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
//         [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
//     ]
// };

pub fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0f32],
        [0.0, 1.0, 0.0, 0.0f32],
        [0.0, 0.0, 1.0, 0.0f32],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

pub fn perspective_matrix(terminal_size: (u32, u32)) -> [[f32; 4]; 4] {
    let (width, height) = terminal_size;
    let width = width as u16;
    let height = height as u16;
    let aspect_ratio = f32::from(height) / f32::from(width);

    let fov: f32 = std::f32::consts::PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

pub fn view_matrix(position: &[f32; 3], rotation: &[f32; 3]) -> [[f32; 4]; 4] {
    let mut m = [
        [1.0, 0.0, 0.0, 0.0f32],
        [0.0, 1.0, 0.0, 0.0f32],
        [0.0, 0.0, 1.0, 0.0f32],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    m = mat_mul!(rotate_x(rotation[0]), m);
    m = mat_mul!(rotate_y(rotation[1]), m);
    m = mat_mul!(rotate_z(rotation[2]), m);
    m = mat_mul!(translate(-position[0], -position[1], -position[2]), m);

    m
}
