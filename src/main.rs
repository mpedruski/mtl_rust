extern crate mtl;

fn main() {
    // Read data into memory
    let data = mtl::gen_data();
    let location_count = data.len();
    // Choose the appropriate tour
    let tour = mtl::initiate_program();
    // Spatial walking tour and functions
    if tour == 1 {
        mtl::spatial_walking_tour(location_count, &data);
    } else {
        mtl::temporal_walking_tour(location_count, &data);
    }
}
