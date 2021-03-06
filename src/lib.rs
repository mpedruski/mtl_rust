use std::io;
use std::collections::HashSet;
use rand::Rng;

// Data structure

pub struct Location {
    pub name: String,
    pub long: f64,
    pub lat: f64,
    pub quartier: String,
    pub story: String,
    pub year: i32,
    pub month: i32,
}

impl Location {
    pub fn output(&self) -> () {
        println!("");
        println!("{}",self.name);
        println!("");
        println!("-{}-",self.year);
        println!("");
        println!("{}",self.story);
    }
}

// Functions common to spatial and temporal walking tours

pub fn initiate_program() -> usize {
    "Prints welcome messages and allows user to choose which tour to take";
    println!("Welcome to Michael's walking tour of Montréal");
    println!("Please choose the kind of tour you would like to be on:");
    println!("1) A spatial walking tour");
    println!("2) A temporal walking tour");
    println!("Please enter your choice as an integer.");
    // Needs input sanitization
    let options = vec![String::from("1"), String::from("2")];
    let choice = numerical_input(options);
    choice
}

pub fn newline_character_sanitization(text: String) -> String {
    "Removes potential newline characters from user input strings";
    let mut input = text;
    if let Some('\n')=input.chars().next_back() { //remove newline characters
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() { //remove newline characters (windows)
        input.pop();
    }
    input
}

pub fn numerical_input(options: Vec<String>) -> usize {
    "Accepts a vector of possible inputs (must be a vector of strings of only an integer and
    newline character), parses the string, and returns the included integer";
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)
            .expect("Couldn't read line");
        input = newline_character_sanitization(input); // removes unix/windows newline characters
        if options.contains(&input){
            break
        }
        println!("Oops, looks like that wasn't an allowable input. Try again!\n");
        input = String::new();
    }
    let choice = input.trim().parse::<usize>().unwrap();
    println!("");
    choice
}

pub fn keep_going() -> bool {
    "Prompts the user to indicate whether they want to quit or not, and returns a boolean";
    println!("To quit type q + Enter. To continue the tour type any key + Enter");
    // Read and parse input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read line");
    input = newline_character_sanitization(input);

    // Generate bool from input
    let choice =
        if input == String::from("q"){
            false
        } else {
            true
        };
    choice
}

pub fn variable_options(x: usize) -> Vec<String> {
    let mut v = Vec::<String>::new();
    for i in 0..x {
        v.push(format!("{}",i));
    }
    v
}

// Spatial walking tour functions

pub fn spatial_walking_tour(location_count: usize, data: &Vec<Location>) {
    // Choose initial location based on neighbourhood
    let mut choice = initiate_spatial_tour(location_count, &data);
    // current_location is noted to facilitate easier navigation later (new call to next_step)
    let mut current_location = choice;
    loop {
        let location = &data[choice];
        location.output();
        println!("");
        let continue_tour = keep_going();
        if continue_tour == false {
            break
        }
        choice = next_step_spatial(choice, location_count, &data); // index of next location chosen by user
        println!("");
        loop {
            if choice != current_location { // if the next location isn't the current, reset current location
                current_location = choice;
                break
            }
            // while next location (choice) is current location keep asking user to choose a next location
            choice = next_step_spatial(choice, location_count, &data);
        }
    }
}

pub fn initiate_spatial_tour(location_count: usize, data: &Vec<Location>) -> usize {
    "Determines the first location in the spatial walking tour by presenting the user with the
    list of unique neighbourhoods, and asking them to choose which neighbourhood to visit. Then
    returns the index of the first location from that neighbourhood";

    println!("What neighbourhood would you like to visit? \
    (Enter your choice as the number before the dash)");

    // List all the neighbourhoods that could be visited
    let mut neighbs: HashSet<String> = vec![String::from(&data[0].quartier)].into_iter().collect();
    for i in 1..location_count {
        neighbs.insert(String::from(&data[i].quartier));
    }
    // Present the user with the choices
    let v: Vec<_> = neighbs.into_iter().collect();
    for i in 0..v.len() {
        println!("{} - {}",i, v[i]);
    }

    // User chooses which neighbourhood to visit with numerical input
    let options = variable_options(v.len());
    // let options = vec![String::from("0\n"),String::from("1\n"),String::from("2\n")];
    let choice = numerical_input(options);
    // Determine the name of the neighbourhood that is indicated by numerical input
    let mut i = 0;
    let quartier = loop {
        let response = &v[i];
        if i == choice {
            break
            response
        }
        i += 1;
    };
    println!("You chose to visit the neighbourhood: {}",quartier);
    // Choose first location from data from that neighbourhood
    let mut i = 0;
    let next_location = loop {
        if &data[i].quartier == quartier {
            break
            i
        }
        i += 1;
    };
    next_location
}

pub fn next_step_spatial(current_location: usize, location_count: usize, data: &Vec<Location>) -> usize {
    "Presents the user with 6 ways to move around spatially, and returns the index of the next
    location based on the chosen modality (note that in the case where the user can't move in
    the requested way from the current location it returns the current location and will be called
    again from main.rs)";

    let options = vec!["1) I want to go north.","2) I want to go east.","3) I want to go south.",
        "4) I want to go west.", "5) Take me to whatever is closest.", "6) I don't care, take me anywhere."];
    let options_numeric = vec![String::from("1"),String::from("2"),String::from("3"),
        String::from("4"),String::from("5"),String::from("6")];

    println!("What do you want to do now?");

    let mut input = String::new();
    loop {
        for i in &options {
            println!("{}",i);
        }
        println!("");
        io::stdin().read_line(&mut input)
            .expect("Couldn't read line");
        input = newline_character_sanitization(input);
        if options_numeric.contains(&input) {
            break
        }
        println!("Oops, that input isn't allowed. Try again!\n");
        input = String::new();
    }

    let next_location =
        if input == String::from("1") {
            move_north_south(location_count, current_location, data, 1)
        } else if input == String::from("2") {
            move_east_west(location_count, current_location, data, 1)
        } else if input == String::from("3") {
            move_north_south(location_count, current_location, data, 0)
        } else if input == String::from("4") {
            move_east_west(location_count, current_location, data, 0)
        } else if input == String::from("5") {
            // println!("{}",current_location);
            let arr = euclidean_dist(location_count, data);
            next_location_euclidean(current_location, arr)
        } else {
            let mut potential = rand::thread_rng().gen_range(0, location_count-1);
            loop {
                if potential != current_location {
                    break
                }
                potential = rand::thread_rng().gen_range(0, location_count-1);
            };
            potential
        };
    next_location
}

pub fn euclidean_dist(location_count: usize, data: &Vec<Location>) -> ndarray::Array2<f64> {
    "Generates an array of the euclidean distances between each location based on latitude and
    longitude. Supplies an aribtrarily large value for the distance of a location to itself";

    let mut arr = ndarray::Array2::<f64>::ones((location_count, location_count));
    for i in 0..location_count {
        for j in 0..location_count {
            if i == j {
                arr[[i,j]] = 100.0; // arbitrarily large value
            } else {
            arr[[i,j]] = ((data[i].lat - data[j].lat).powf(2.0) + (data[i].long - data[j].long).powf(2.0)).powf(0.5);
            }
        }
    }
    // println!("{}",arr);
    arr
}

pub fn next_location_euclidean(current_location: usize, arr: ndarray::Array2<f64>) -> usize {
    "Returns the index of the closest location to the current location based on euclidean distance";

    // let possible_locations = arr
    let possible_locations = arr.slice(ndarray::s![current_location, ..]);
    let mut min_distance = 100.0; // arbitrarily large value
    let mut index = current_location;
    for i in 0..possible_locations.len() {
        // println!("{}",possible_locations[[i]]);
        if possible_locations[i] < min_distance {
            min_distance = possible_locations[i];
            // println!("Min distance updated: {}",min_distance);
            index = i;
            // println!("Index updated: {}",index);
        };
    };
    // println!("Final index: {}",index);
    index
}

pub fn move_north_south(location_count: usize, current_location: usize,
        data: &Vec<Location>, mode:i32) -> usize {
    "Returns the index of the next location north (mode == 1) or south (mode == 0) of the current
    location if such a location exists (otherwise returns current location)";
    let current_latitude = data[current_location].lat;
    let mut smallest_increment = 100.0; // arbitrarily large value
    let mut next_location = current_location;
    let mut direction = 1.0;
    if mode == 0 {
        direction = -1.0;
    }
    for i in 0..location_count {
        if direction*(data[i].lat - current_latitude) < smallest_increment &&
            direction*(data[i].lat - current_latitude) > 0.0 {
            smallest_increment = direction*(data[i].lat - current_latitude);
            next_location = i;
        };
    };
    if current_location == next_location {
        println!("Oops, you can't go that way! Try another direction!\n");
    }
    next_location
}

pub fn move_east_west(location_count: usize, current_location: usize,
        data: &Vec<Location>, mode:i32) -> usize {
    "Returns the index of the next location east (mode == 1) or west (mode == 0) of the current
    location if such a location exists (otherwise returns current location)";

    let current_longitude = data[current_location].long;
    let mut smallest_increment = 100.0; // arbitrarily large value
    let mut next_location = current_location;
    let mut direction = 1.0;
    if mode == 0 {
        direction = -1.0;
    }
    for i in 0..location_count {
        if direction*(data[i].long - current_longitude) < smallest_increment &&
            direction*(data[i].long - current_longitude) > 0.0 {
            smallest_increment = direction*(data[i].long - current_longitude);
            next_location = i;
        };
    };
    if current_location == next_location {
        println!("Oops, you can't go that way! Try another direction!\n");
    }
    next_location
}

// Temporal walking tour functions

pub fn temporal_walking_tour(location_count: usize, data: &Vec<Location>, timepoints: Vec<i32>) {

    println!("In what year would you like to start your tour? \
    (Enter your choice as the number before the dash)");
    let mut choice = initiate_temporal_tour(location_count, data);
    // current_location is noted to facilitate easier navigation later (new call to next_step)
    let mut current_location = choice;
    loop {
        let location = &data[choice];
        location.output();
        println!("");
        let continue_tour = keep_going();
        if continue_tour == false {
            break
        }
        choice = next_step_temporal(choice, location_count, &data, &timepoints); // index of next location chosen by user
        println!("");
        loop {
            if choice != current_location { // if the next location isn't the current, reset current location
                current_location = choice;
                break
            }
            // while next location (choice) is current location keep asking user to choose a next location
            choice = next_step_temporal(choice, location_count, &data, &timepoints);
        }
    }
}

pub fn initiate_temporal_tour(location_count: usize, data: &Vec<Location>) -> usize {
    // List all the neighbourhoods that could be visited
    let mut years: HashSet<&i32> = vec![&data[0].year].into_iter().collect();
    for i in 1..location_count {
        years.insert(&data[i].year);
    }
    // Present the user with the choices
    let mut v: Vec<_> = years.into_iter().collect();
    v.sort();
    for i in 0..v.len() {
        println!("{} - {}",i, v[i]);
    }

    // User chooses which year to visit with numerical input
    let options = variable_options(v.len());
    let choice = numerical_input(options);
    // Determine the name of the year that is indicated by numerical input
    let mut i = 0;
    let year = loop {
        let response = &v[i];
        if i == choice {
            break
            response
        }
        i += 1;
    };
    println!("You chose to visit the year: {}", year);
    // Choose first location from data from that neighbourhood
    let mut i = 0;
    let next_location = loop {
        if &data[i].year == *year {
            break
            i
        }
        i += 1;
    };

    next_location
}

pub fn next_step_temporal (current_location: usize, location_count: usize, data: &Vec<Location>,
    timepoints: &Vec<i32>) -> usize {
    "Presents the user with 6 ways to move around temporally, and returns the index of the next
    location based on the chosen modality (note that in the case where the user can't move in
    the requested way from the current location it returns the current location and will be called
    again from temporal_walking_tour)";

    let options = vec!["1) I want to go forward in time.","2) I want to go backward in time.",
    "3) I want to visit winter.", "4) I want to go spring.", "5) I want to visit summer.",
    "6) I want to visit fall."];
    let options_numeric = vec![String::from("1"),String::from("2"),String::from("3"),
        String::from("4"),String::from("5"),String::from("6")];

    println!("What do you want to do now?");

    let mut input = String::new();
    loop {
        for i in &options {
            println!("{}",i);
        }
        println!("");
        io::stdin().read_line(&mut input)
            .expect("Couldn't read line");
        input = newline_character_sanitization(input);
        if options_numeric.contains(&input) {
            break
        }
        println!("Oops, that input isn't allowed. Try again!\n");
        input = String::new();
    }

    let next_location =
        if input == String::from("1") {
            forward_backward(current_location, location_count, timepoints, 1)
        } else if input == String::from("2") {
            forward_backward(current_location, location_count, timepoints, 0)
        } else if input == String::from("3") {
            random_season_selector(current_location, location_count, &data, 0)
        } else if input == String::from("4") {
            random_season_selector(current_location, location_count, &data, 1)
        } else if input == String::from("5") {
            random_season_selector(current_location, location_count, &data, 2)
        } else {
            random_season_selector(current_location, location_count, &data, 3)
        };
    next_location
}

pub fn random_season_selector(current_location: usize, location_count: usize, data: &Vec<Location>,
    mode: i32) -> usize {
    // Create a vector with the suitable months for each season
    let mut months = vec![9, 10, 11];
    if mode == 0 {
        months = vec![12, 1, 2];
    } else if mode == 1 {
        months = vec![3,4,5];
    } else if mode == 2 {
        months = vec![6,7,8];
    } else {
        months = months;
    };
    // Generate a vector with all the memories from the selected season
    let mut seasonal_locations = Vec::new();
    for i in 0..location_count {
        if months.contains(&data[i].month){
            seasonal_locations.push(i);
        }
    }
    // Choose a random index for the vector, and return the location associated with that index
    // Will cause panic if there isn't at least one memory for the selected season
    let chosen = rand::thread_rng().gen_range(0, seasonal_locations.len());
    if current_location == seasonal_locations[chosen] {
        println!("Oops, looks like we led you back to your current location. Pick your next move!")
    }
    seasonal_locations[chosen]
}

pub fn gen_timepoints(location_count: usize, data: &Vec<Location>) -> Vec<i32> {
    let mut timepoints = Vec::new();
    for i in 0..location_count {
        timepoints.push((data[i].year - 2002)*12 + data[i].month);
    }
    timepoints
}

pub fn forward_backward(current_location: usize, location_count: usize, timepoints: &Vec<i32>,
        mode: i32) -> usize {
    let current_timepoint = timepoints[current_location];
    let mut min_difference = 100; // arbitrarily large value
    let mut direction = 1;
    if mode == 0 {
        direction = -1;
    }
    let mut next_location = current_location;
    for i in 0..location_count {
        if direction*(timepoints[i] - current_timepoint) < min_difference &&
            direction*(timepoints[i] - current_timepoint) > 0 {
            min_difference = direction*(timepoints[i] - current_timepoint);
            next_location = i;
        }
    };
    if current_location == next_location {
        println!("Oops, you can't go that way! Try another option!\n");
    }
    next_location
}
