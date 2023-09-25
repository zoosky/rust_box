pub struct ParsedData {
    pub header: u8,
    pub payload: String,
}

impl ParsedData {
    pub fn parse(data: &[u8]) -> ParsedData {
        let header = data[0];

        let payload_bytes = data[1..data.len()].to_vec();
        let payload = String::from_utf8(payload_bytes).unwrap();

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