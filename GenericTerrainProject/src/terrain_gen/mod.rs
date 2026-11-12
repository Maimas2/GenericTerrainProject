// use noise::Perlin;

#[derive(Debug)]
pub struct MeshRawData {
    pub points: Vec<f32>,
    pub indices: Vec<u32>
}

pub fn generate_test_mesh() -> Vec<f32> {
    vec![
        0., 0.5, 0.5, 1., 1., 0.,
        0., -0.5, -0.5, 1., 1., 0.,
        0., 0.5, -0.5, 1., 0., 0.,
        0., -0.5, 0.5, 1., 1., 0.,
        ]
}

pub fn generate_mesh() -> MeshRawData {
    MeshRawData {
        points: vec![
            0., 0.5, 0.5, 1., 1., 0.,
            0., -0.5, -0.5, 1., 1., 0.,
            0., 0.5, -0.5, 1., 0., 0.,
            0., -0.5, 0.5, 1., 1., 0.,
        ],
        indices: vec![0, 1, 2, 0, 1, 3]
    }
}