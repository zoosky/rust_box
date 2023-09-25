pub struct ParsedData<'a> {
    pub header: u8,
    pub payload: &'a str,
}

impl<'a> ParsedData<'a> {
    pub fn parse(data: &'a [u8]) -> ParsedData<'a> {
        let header = data[0];

        let payload = std::str::from_utf8(&data[1..data.len()]).unwrap();

        ParsedData { header, payload }
    }
}

fn get_data() -> Vec<u8> {
    const DATA: [u8; 5] = [255, 't' as u8, 'e' as u8, 's' as u8, 't' as u8];
    DATA.to_vec() // Return dynamically allocated array (Vector)
}

fn main() {
    // Simulate getting data from somewhere else (Ex: Socket) (Rust allows us to return a object)
    let buffer = get_data();

    // Parse buffer into ParsedData struct
    let parsed_data = ParsedData::parse(&buffer);

    // Print payload content
    println!("{}", parsed_data.payload);
}
