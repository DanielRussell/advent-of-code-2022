// Find the elf carrying the most Calories
use std::io;

fn main() {
    let mut most_calories = 0;
    let mut this_calories = 0;
    
    loop {
        let mut buffer = String::new();
        if let Ok(nread) = io::stdin().read_line(&mut buffer) {
            if nread == 0 {
                break;
            }
        }
        // println!("buffer: {}", buffer);

        if buffer.trim() == "" {
            println!("elf with {} calories", this_calories);
            if this_calories > most_calories {
                most_calories = this_calories;
            }
            this_calories = 0;
        } else {
            let cal = buffer.trim().parse::<i32>().unwrap();
            // println!("{} calories", cal);
            this_calories += cal;
        }
    }
    println!("-> {} <-", most_calories);
}
