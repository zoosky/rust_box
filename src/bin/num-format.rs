use num_format::Grouping;
use num_format::{Locale, ToFormattedString};

fn main() {
    let s = 1000000.to_formatted_string(&Locale::en);
    assert_eq!(&s, "1,000,000");

    let ch: String = 1234567890.to_formatted_string(&Locale::de_CH);
    assert_eq!(&ch, "1’234’567’890");

    locale_test()
}

fn locale_test() {
    let locale = Locale::en;
    assert_eq!(locale.grouping(), Grouping::Standard);
    assert_eq!(locale.minus_sign(), "-");
    assert_eq!(locale.name(), "en");
    assert_eq!(locale.separator(), ",");

    let locale2 = Locale::from_name("en").unwrap();
    assert_eq!(locale, locale2);

    let available = Locale::available_names();
    println!("All of the locale names available in the Unicode database are...");
    println!("{:#?}", available);
}
