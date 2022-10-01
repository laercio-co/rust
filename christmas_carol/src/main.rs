fn main() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts: [&str; 12] = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            if i < 1 {
                println!("A partridge in a pear tree");
            } else {
                println!("{}", gifts[j]);
            }
        }
    }
}
