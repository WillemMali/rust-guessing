use std::io;

fn main() {
    struct Difficulty<'a> {
        id: i32,
        name: &'a str,
        min: i32,
        max: i32,
    }

    let difficulties = [
        Difficulty {
            id: 0,
            name: "-",
            min: 0,
            max: 0
        },
        Difficulty {
            id: 1,
            name: "easy-peasy",
            min: 1,
            max: 2,
        },
        Difficulty {
            id: 2,
            name: "normal",
            min: 1,
            max: 5,
        },
        Difficulty {
            id: 3,
            name: "tricky",
            min: 1,
            max: 10,
        },
        Difficulty {
            id: 4,
            name: "hard",
            min: 1,
            max: 25,
        },
        Difficulty {
            id: 5,
            name: "impossibru",
            min: 1,
            max: 50,
        },
    ];


    let mut difficulty: usize = 0;

    while difficulty == 0
    {
        println!("Select difficulty:");

        // List difficulties
        for difficulty in difficulties.iter() {
            println!(
                "    {}: {} ({} - {})",
                difficulty.id,
                difficulty.name,
                difficulty.min,
                difficulty.max);
        }

        println!("\nDifficulty:");

        let mut answer = String::new();
        
        io::stdin().read_line(&mut answer)
            .expect("Failed to read line");

        println!("{}", answer);

        answer = answer.replace("\n", "");
        
        match answer.parse::<usize>() {
            Ok(value) => difficulty = value,
            Err(error) => println!(
                "Could not parse your input! Error message: {}", error),
        }
    }

    println!("You chose difficulty: {}", difficulty);
    println!("Guess a number between 1 and {}: ", difficulty);
}
