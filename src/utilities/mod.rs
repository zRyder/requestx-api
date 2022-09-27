pub mod users;

pub fn generate_numeric_id(size: usize) -> u32 {
    let range: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    nanoid::nanoid!(size, &range).parse::<u32>().unwrap()
}

pub fn generate_id(size: usize) -> String {
    nanoid::nanoid!(size).parse::<String>().unwrap()
}
