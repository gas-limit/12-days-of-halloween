fn main() {
    let day = ["first", "second", "third",
               "fourth", "fifth", "sixth",
               "seventh", "eighth", "ninth",
               "tenth", "eleventh", "twelfth"];
    let gift = ["A Vulture in a dead tree",
                "Two hissing cats",
                "Three fat toads",
                "Four giggling ghosts",
                "Five cooked worms",
                "Six owls a-screeching",
                "Seven spiders creeping",
                "Eight brooms a-flying",
                "Nine wizards whizzing",
                "Ten goblins gobbling",
                "Eleven bats a-swooping",
                "Twelve cauldrons bubbling"];

    let mut day_count = 0;
  
    for i in 0..day.len() {
        println!("For the {} of Halloween, my good friend gave to me", day[i]);

        for j in (0..=day_count).rev() {
            if j == 0 && day_count != 0 {
                println!("and {}", gift[j]);
            } else {
                println!("{}", gift[j]);
            }
        }

        day_count += 1;
    }
}
