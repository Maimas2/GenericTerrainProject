use noise::{NoiseFn, OpenSimplex};

#[derive(Debug)]
pub struct MeshRawData {
    pub points: Vec<f32>,
    pub indices: Vec<u32>,
}

const LOD: u32 = 5;

const NOISE_SCALING: f64 = 1.;

pub fn generate_test_mesh() -> MeshRawData {
    MeshRawData {
        points: vec![
            0., 0.5, 0.5,    1., 1., 0.,   0., 0., 1.,
            0., -0.5, -0.5,  1., 1., 0.,   0., 0., 1.,
            0., 0.5, -0.5,   1., 0., 0.,   0., 0., 1.,
            0., -0.5, 0.5,   1., 1., 0.,   0., 0., 1.,
        ],
        indices: vec![0, 1, 2, 0, 1, 3]
    }
}

pub fn generate_mesh(noise_seed: u32, chunk_pos: [i32; 2], chunk_size: f32) -> MeshRawData {
    let noise_object: OpenSimplex = OpenSimplex::new(noise_seed);

    let chunk_offset: [f64; 2] = [chunk_pos[0] as f64 * chunk_size as f64, chunk_pos[1] as f64 * chunk_size as f64];

    let mut raw_points: Vec<Vec<f32>> = Vec::new();
    let size_measure: usize = 2_usize.pow(LOD) + 1;
    let inv_size_measure: f32 = chunk_size / (2_f32.powf(LOD as f32));

    for x in 0..size_measure {
        raw_points.push(Vec::new());
        for y in 0..size_measure {
            let arg: [f64; 2] = [((x as f64) * (inv_size_measure as f64) + chunk_offset[0]) * NOISE_SCALING, ((y as f64) * (inv_size_measure as f64) + chunk_offset[1]) * NOISE_SCALING];

            let val = noise_object.get(arg);
            
            raw_points.get_mut(x).unwrap().push(val as f32);
        }
    }

    let mut mesh_point_data: Vec<f32> = Vec::new();

    for x in 0..size_measure {
        for y in 0..size_measure {
            let temp_points: [f32; 3] = [(x as f32) * inv_size_measure, raw_points[x][y] * 0.2 * chunk_size, (y as f32) * inv_size_measure];

            mesh_point_data.push(temp_points[0]);
            mesh_point_data.push(temp_points[1]);
            mesh_point_data.push(temp_points[2]);

            mesh_point_data.push(1.);
            mesh_point_data.push((x as f32) / ((size_measure-1) as f32));
            mesh_point_data.push((y as f32) / ((size_measure-1) as f32));

            // Temp holder normal data. To be overwritten after index creation.
            mesh_point_data.push(1.);
            mesh_point_data.push(0.);
            mesh_point_data.push(0.);
        }
    }

    let mut mesh_index_data: Vec<u32> = Vec::new();

    for x in 0..size_measure-1 {
        for y in 0..size_measure-1 {
            let initial_corner: u32 = (x + y*size_measure) as u32;
            mesh_index_data.push(initial_corner);
            mesh_index_data.push(initial_corner+1);
            mesh_index_data.push(initial_corner+(size_measure as u32)+1);

            mesh_index_data.push(initial_corner);
            mesh_index_data.push(initial_corner+(size_measure as u32));
            mesh_index_data.push(initial_corner+(size_measure as u32)+1);
        }
    }

    let mut actual_to_return: Vec<f32> = Vec::new();
    for ind in mesh_index_data.iter() {
        for i in 0..9 {
            actual_to_return.push(*mesh_point_data.get(((ind*9)+i) as usize).unwrap());
        }
    }

    // for x in 0..size_measure-1 {
    //     for y in 0..size_measure-1 {
    //         let start_mesh_index: u32 = (x + y*size_measure) as u32 * 9;
    //         let start_index_index: u32 = (x + y*(size_measure-1)) as u32 * 6;

    //         let point_1: Vector3<f32> = Vector3::new(
    //             mesh_point_data[(start_mesh_index+6) as usize],
    //             mesh_point_data[(start_mesh_index+7) as usize],
    //             mesh_point_data[(start_mesh_index+8) as usize],
    //         );
    //         let point_2: Vector3<f32> = Vector3::new(
    //             mesh_point_data[(start_mesh_index+9+6) as usize],
    //             mesh_point_data[(start_mesh_index+9+7) as usize],
    //             mesh_point_data[(start_mesh_index+9+8) as usize],
    //         );
    //         let point_3: Vector3<f32> = Vector3::new(
    //             mesh_point_data[(start_mesh_index+18+6) as usize],
    //             mesh_point_data[(start_mesh_index+18+7) as usize],
    //             mesh_point_data[(start_mesh_index+18+8) as usize],
    //         );
    //     }
    // }
    
    MeshRawData {
        points: actual_to_return,
        indices: mesh_index_data
        // indices: vec![0, 1, 3, 0, 2, 3]
    }
}