fn main() {
    // Credits to the code: krona's medium blog
    let days = ["first", "second", "third", "fourth", "fifth",
                "sixth", "seventh", "eighth", "ninth", "tenth", 
                "eleventh", "twelfth"];
    
    for (day_num, day) in days.iter().enumerate() {
        println!("For the {} day of Christmas my true love sent to me", day);

        for gift_day in (1..(day_num + 1)).rev(){
            if gift_day == 1 && day_num != 1 {
                println!("and ");
            }
            if gift_day == 1 {
                println!("a Patridge in a Pear Tree");
            } else if gift_day == 2 {
                println!("Two Turtle Doves");
            } else if gift_day == 3 {
                println!("Three French Hens");
            } else if gift_day == 4 {
                println!("Four Calling Birds");
            } else if gift_day == 5 {
                println!("Five Golden Rings");
            } else if gift_day == 6 {
                println!("Six Geese a Laying");
            } else if gift_day == 7 {
                println!("Seven Swans a Swimming");
            } else if gift_day == 8 {
                println!("Eight Maids a Milking");
            } else if gift_day == 9 {
                println!("Nine Ladies Dancing");
            } else if gift_day == 10 {
                println!("Ten Lords a Leaping");
            } else if gift_day == 11 {
                println!("Eleven Pipers Piping");
            } else if gift_day == 12 {
                println!("12 Drummers Drumming");
            }
        }
        
        println!("\n");
    }
}
