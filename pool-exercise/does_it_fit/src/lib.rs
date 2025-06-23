pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let max_size = x * y;
    let size;
    match objects {
        GeometricalShapes::Square => size = square_area(a) as f64,
        GeometricalShapes::Circle => size = circle_area(a),
        GeometricalShapes::Rectangle => size = rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => size = triangle_area(a, b),
    }
    times as f64 * size <= max_size as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let max_size = x * y * z;
    let size;
    match objects {
        GeometricalVolumes::Cube => size = cube_volume(a) as f64,
        GeometricalVolumes::Sphere => size = sphere_volume(a),
        GeometricalVolumes::Parallelepiped => size = parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Pyramid => size = triangular_pyramid_volume(triangle_area(a, b), c),
        GeometricalVolumes::Cone => size = cone_volume(a, b),
    }
    times as f64 * size <= max_size as f64
}