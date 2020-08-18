// Raw data
use mtl::Location;

pub fn gen_data() -> Vec<Location> {
    "Generates a vector of location structs from individual struct instances";

    let loc0 = Location {
        name: String::from("Westmount Park"),
        long: -73.59717,
        lat: 45.48146,
        quartier: String::from("Westmount"),
        story: String::from("On a nice sunny day I sat here and watched the ducks"),
        year: 2014,
        month: 06,
    };

    let loc1 = Location {
        name: String::from("L'Oratoire Saint-Joseph"),
        long: -73.61749,
        lat: 45.49234,
        quartier: String::from("Côte des Neiges"),
        story: String::from("Once upon a time I went to the Oratory"),
        year: 2019,
        month: 09,
    };

    let loc2 = Location {
        name: String::from("Chalet du Mont-Royal"),
        long: -73.58721,
        lat: 45.50374,
        quartier: String::from("Ville-Marie"),
        story: String::from(
            "I ran to the chalet in March. It was pretty cold, but sunny, so I \
            couldn't really complain. Frankly, I had a good time."),
        year: 2020,
        month: 03,
    };

    let loc3 = Location {
        name: String::from("Westmount Library"),
        long: -73.59920,
        lat: 45.48185,
        quartier: String::from("Westmount"),
        story: String::from("Once upon a time I went to the Westmount library and read a book"),
        year: 2016,
        month: 12,
    };

    let loc4 = Location {
        name: String::from("Westmount Library: II"),
        long: -73.59921,
        lat: 45.48184,
        quartier: String::from("Westmount"),
        story: String::from("It's Bloomsday at the Wesmount Library!"),
        year: 2019,
        month: 06,
    };

    let data = vec![loc0, loc1, loc2, loc3, loc4];

    data
}
