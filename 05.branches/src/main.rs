fn main() {
    let lyrics = ["And a partridge in a pear tree.",
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
                            "Twelve drummers drumming,",];
    
    for line_number in 1..lyrics.len()+1 {
        let suffix = if line_number == 1 {"st"} 
                            else if line_number == 2 {"nd"}
                            else if line_number == 3 {"rd"}
                            else {"th"};
        println!("\nOn the {line_number}{suffix} day of Christmas, my true love gave to me...");
        for line in lyrics[0..line_number].iter().rev() {
            println!("{line}");
        }
    }
}