extern crate mtl;

fn main() {
    // Read data into memory
    let data = mtl::gen_data();
    let location_count = data.len();
    // Choose the appropriate tour
    let tour = mtl::initiate_program();
    // Spatial walking tour and functions
    if tour == 1 {
        // Choose initial location based on neighbourhood
        let mut choice = mtl::initiate_walking_tour(location_count, &data);
        // current_location is noted to facilitate easier navigation later (new call to next_step)
        let mut current_location = choice;
        loop {
            let location = &data[choice];
            location.output();
            println!("");
            let continue_tour = mtl::keep_going();
            if continue_tour == false {
                break
            }
            choice = mtl::next_step(choice, location_count, &data); // index of next location chosen by user
            println!("");
            loop {
                if choice != current_location { // if the next location isn't the current, reset current location
                    current_location = choice;
                    break
                }
                // while next location (choice) is current location keep asking user to choose a next location
                choice = mtl::next_step(choice, location_count, &data);
            }
        }
    } // end spatial walking tour
}
