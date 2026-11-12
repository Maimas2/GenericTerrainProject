use noise::{NoiseFn, Perlin};

#[derive(Debug)]
pub struct MeshRawData {
    pub points: Vec<f32>,
    pub indices: Vec<u32>,
}

const LOD: u32 = 5;
const CHUNK_WORLD_SIZE: f32 = 2.;

pub fn generate_test_mesh() -> MeshRawData {
    MeshRawData {
        points: vec![
            0., 0.5, 0.5,   1., 1., 0., 
            0., -0.5, -0.5, 1., 1., 0., 
            0., 0.5, -0.5,  1., 0., 0., 
            0., -0.5, 0.5,  1., 1., 0., 
        ],
        indices: vec![0, 1, 3, 0, 2, 3]
    }
}

pub fn generate_mesh(noise_seed: u32) -> MeshRawData {
    let noise_object: Perlin = Perlin::new(noise_seed);

    let mut raw_points: Vec<Vec<f32>> = vec![];
    let size_measure: usize = 2_usize.pow(LOD) + 1;
    let inv_size_measure: f32 = CHUNK_WORLD_SIZE / (2_f32.powf(LOD as f32));

    for x in 0..size_measure {
        raw_points.push(vec![]);
        for y in 0..size_measure {
            // let arg: [f64; 3] = [42.4, 37.8, 2.8];
            let arg: [f64; 2] = [(x as f64) * (inv_size_measure as f64), (y as f64) * (inv_size_measure as f64)];
            let val = noise_object.get(arg);
            
            raw_points.get_mut(x).unwrap().push(val as f32);
        }
    }

    let mut mesh_point_data: Vec<f32> = vec![];

    for x in 0..size_measure {
        for y in 0..size_measure {
            mesh_point_data.push((x as f32) * inv_size_measure);
            mesh_point_data.push(raw_points[x][y] * 0.5);
            mesh_point_data.push((y as f32) * inv_size_measure);

            mesh_point_data.push(1.);
            mesh_point_data.push((x as f32) / ((size_measure-1) as f32));
            mesh_point_data.push((y as f32) / ((size_measure-1) as f32));
        }
    }

    let mut mesh_index_data: Vec<u32> = vec![];

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

    // println!("{:?}", mesh_point_data);
    
    MeshRawData {
        points: mesh_point_data,
        indices: mesh_index_data
        // indices: vec![0, 1, 3, 0, 2, 3]
    }
}