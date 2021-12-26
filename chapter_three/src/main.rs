fn main() {
    println!("{}°F is {}°C", 50, far_to_cel(50.));
    println!("{}°C is {}°F", 50, cel_to_far(50.));
    println!("The {}th Fibonacci number is {}", 10, fibonacci(10));
    twelve_days();
}

fn far_to_cel(t_far: f64) -> f64 {
    (t_far - 32.) / 1.8
}

fn cel_to_far(t_cel: f64) -> f64 {
    1.8 * t_cel + 32.
}

fn fibonacci(n: u32) -> u32 {
    let mut f_n_2 = 0;
    let mut f_n_1 = 1;
    let mut f_n = f_n_1 + f_n_2;
    match n {
        0 => f_n_2,
        1 => f_n_1,
        _ => {
            for _ in 2..n {
                f_n_2 = f_n_1;
                f_n_1 = f_n;
                f_n = f_n_1 + f_n_2;
            }
            f_n
        }
    }
}

fn twelve_days() {
    println!("");
    println!("On the first day of Christmas");
    println!("My true love sent to me");
    println!("A partridge in a pear tree");

    // let mut lyrics = ["", "On the second day of Christmas", "My true love sent to me", "Two turtle-doves", "And a partridge in a pear tree"]

    // for lyric in lyrics {
    //     println!(lyric);
    // }
    println!("");
    println!("On the second day of Christmas");
    println!("My true love sent to me");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the third day of Christmas");
    println!("My true love sent to me");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the fourth day of Christmas");
    println!("My true love sent to me");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the fifth day of Christmas");
    println!("My true love sent to me");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the sixth day of Christmas");
    println!("My true love sent to me");
    println!("Six geese a laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the seventh day of Christmas");
    println!("My true love sent to me");
    println!("Seven swans a swimming");
    println!("Six geese a-laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the eighth day of Christmas");
    println!("My true love sent to me");
    println!("Eight maids a milking");
    println!("Seven swans a swimming");
    println!("Six geese a-laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the ninth day of Christmas");
    println!("My true love sent to me");
    println!("Nine ladies dancing");
    println!("Eight maids a-milking");
    println!("Seven swans a-swimming");
    println!("Six geese a-laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
    println!("");
    println!("On the tenth day of Christmas");
    println!("My true love sent to me");
    println!("Ten lords a-leaping");
    println!("Nine ladies dancing");
    println!("Eight maids a-milking");
    println!("Seven swans a-swimming");
    println!("Six geese a-laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");

    println!("");
    println!("On the 11th day of Christmas");
    println!("My true love sent to me");
    println!("I sent 11 pipers piping");
    println!("Ten lords a-leaping");
    println!("Nine ladies dancing");
    println!("Eight maids a-milking");
    println!("Seven swans a-swimming");
    println!("Six geese a-laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");

    println!("");
    println!("On the 12th day of Christmas");
    println!("My true love sent to me");
    println!("12 drummers drumming");
    println!("Eleven pipers piping");
    println!("Ten lords a-leaping");
    println!("Nine ladies dancing");
    println!("Eight maids a-milking");
    println!("Seven swans a-swimming");
    println!("Six geese a-laying");
    println!("Five golden rings");
    println!("Four calling birds");
    println!("Three French hens");
    println!("Two turtle-doves");
    println!("And a partridge in a pear tree");
}
