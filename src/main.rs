fn main() {
    let lines:[&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 1..13 {
        let day = match i {
            1 => "first",
            2 => "second",
            3 => "third", 
            4 => "fourth",
            5 => "fifth", 
            6 => "sixth",
            7 => "seventh", 
            8 => "eigth", 
            9 => "ninth",
            10 => "tenth",
            11 => "elevnth",
            12 => "twelfth",
            _ => "",
        };
        println!("\nOn the {} day of Christmas my true love sent to me", day);
        for num in (0..(i)).rev() {
            println!("{}", lines[num]);
        }

    }




}

//         "On the twelfth day of Christmas my true love sent to me",
