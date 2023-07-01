use bevy::{prelude::Mesh};

pub fn mesh_box_new(width:f32,height:f32)->Mesh{
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    let half_width = width*0.5;
    let half_height = height*0.5;
    let positions = vec![
        [half_width,half_height,0.0],
        [half_width,-half_height,0.0],
        [-half_width,-half_height,0.0],
        [-half_width,half_height,0.0],
    ];
    let uv = vec![
        [1.0,1.0],
        [1.0,0.0],
        [0.0,0.0],
        [0.0,1.0],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U16(vec![
        0,1,2,
        2,3,0
    ])));
    return mesh;
}
pub fn mesh_hexagon_new(radius:f32)->Mesh {
    mesh_ngon_new(6, radius)
}
pub fn mesh_ngon_new(points:usize,radius:f32)->Mesh{
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    let mut positions = Vec::with_capacity(points);
    let mut uv= Vec::with_capacity(points);
    let mut ind = vec![];
    let step = (std::f32::consts::PI*2.0) / points as f32;
    for i in 0..points {
        let (y,x) = (step * i as f32).sin_cos();
        uv.push([x*0.5+0.5,y*0.5+0.5]);
        positions.push([x*radius,y*radius,0.0]);
    }
    let half_points = points as u16 / 2;
    for i in 0..(half_points-1) {
        ind.push(i);
        ind.push(i+1);
        ind.push(half_points);
    }
    for i in (half_points)..(points as u16 - 1) {
        ind.push(i);
        ind.push(i+1);
        ind.push(0);
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U16(ind)));
    return mesh;
}