use rand::{thread_rng, Rng};
use clap::Parser;
use rusqlite::{Connection, Result};

#[derive(Debug, Default)]
struct Pokemon {
    name: String,
    id: u32,
    type1: String,
    type2: Option<String>,
    base_hp: u32,
    base_attack: u32,
    base_defense: u32,
    base_sp_attack: u32,
    base_sp_defense:u32,
    base_speed:u32,
    //sprite: idk
    ability: String,
    //matchup: std::collections::HashMap<String, f32>
    moveset: Option<Vec<Move>>,
    total_hp: u32
}

//Maybe but not neccesary
/*impl Pokemon {
    
}*/

//May need to wrap pokemon around move struct
#[derive(Debug)]
struct Move {
    name: String,
    type1: String,
    base_power: u32,
    accuracy: u32,
    category: String,
    priority: u8,
    pp: u8,
    crit: f32,
    description: String
}


fn main() {
    let mut poke1 = Pokemon::default();
    let mut poke2 = Pokemon::default();

    //Get pokemon name and setup its stats
    println!("Enter first pokemon name to retrieve data!");
    poke1 = poke_set(poke1);
    poke1 = move_set(poke1);

    //Do the same for the next mon
    println!("Enter second pokemon name to retrieve data!");
    poke2 = poke_set(poke2);
    poke2 = move_set(poke2);

    //Pokemon and moves are set
    println!("Pokemon are setup, time to start the battle!");

    //Use clap to setup battle inputs (if later updates are desired including swapping and other game mechanics)
    //Only attacking moves are currently used
    //Assumes all pokemon are level 50
    //Calculate pokemon stats
    poke1 = get_stats(poke1);
    poke2 = get_stats(poke2);

    //Beginning of turn logic
    //Wrap in while loop based on hp
    while poke1.total_hp != 0 && poke2.total_hp != 0 {
        //Get first pokemon ready
        println!("What will {:?} do?", poke1.name);

        //Display list of moves for the pokemon
        if let Some(move_vec) = &poke1.moveset {
            for pokemon_move in move_vec {
                println!("- {:?}", pokemon_move.name);     
            }
        } else {
            println!("{:?} has no moves and is unable to battle", poke1.name);
            std::process::exit(1);
        }

        
    }
    


}



//Sets up the pokemon stats based off of base values, neutral nature, and randomized ivs
fn get_stats(mut poke :Pokemon) -> Pokemon {
    //Generate random ivs every time
    let mut rng = thread_rng();

    //Calculate hp
    let mut iv = rng.gen_range(0..=31);
    let mut calc: u32 = ((2 * poke.base_hp + iv) * 50 /100) + 60;
    poke.base_hp = calc;

    //Calculate attack
    iv = rng.gen_range(0..=31);
    calc = (2 * poke.base_attack + iv) * 50 / 100 + 5;
    poke.base_attack = calc;

    //Calculate defense
    iv = rng.gen_range(0..=31);
    calc = (2 * poke.base_defense + iv) * 50 / 100 + 5;
    poke.base_defense = calc;

    //Calculate spA
    iv = rng.gen_range(0..=31);
    calc = (2 * poke.base_sp_attack + iv) * 50 / 100 + 5;
    poke.base_sp_attack = calc;

    //Calculate spD
    iv = rng.gen_range(0..=31);
    calc = (2 * poke.base_sp_defense + iv) * 50 / 100 + 5;
    poke.base_sp_defense = calc;

    //Calculate speed
    iv = rng.gen_range(0..=31);
    calc = (2 * poke.base_speed + iv) * 50 / 100 + 5;
    poke.base_speed = calc;

    return poke;
}



//Take pokemon variable via mutable ref and add to it based on user input
fn poke_set(mut poke :Pokemon) -> Pokemon {
    let mut name_to_search = String::new();
    std::io::stdin()
        .read_line(&mut name_to_search)
        .expect("Failed to read line");

    name_to_search = name_to_search.trim().to_string();
    name_to_search.make_ascii_lowercase();
    let data = get_pokemon(&name_to_search);

    match data {
        Ok(mon) => {
            poke = mon; // Assign the whole struct to first mon
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }

    //Successfully saved struct
    return poke;
}



fn move_set(mut poke :Pokemon) -> Pokemon{
    println!("Let's give {:?} some moves!", poke.name);
    println!("You'll be asked to choose 4.");
    println!("If you wish to skip some simply press enter.");

    //Pick 4 moves
    for x in 1..5 {
        //Add moves
        let mut move_to_search = String::new();
        println!("Selecting move {:?}...", x);
        std::io::stdin()
            .read_line(&mut move_to_search)
            .expect("Failed to read line");
        move_to_search = move_to_search.trim().to_string();
        move_to_search.make_ascii_lowercase();
        println!("You typed {:?}", move_to_search);

        //Check if move already in list maybe later?
        
        //If the user just hit enter then skip to next iteration
        if move_to_search == "" {
            println!("Skipping this move...");
            continue
        }

        let data = get_moves(&move_to_search);
        match data {
            Ok(moves) => {
                poke.moveset.get_or_insert_with(|| vec![]).push(moves);
                println!("Added {:?} to the move list", move_to_search);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
    println!("Moves: {:?}", poke.moveset);
    return poke;
}



fn get_pokemon(name: &String) -> Result<Pokemon, rusqlite::Error> {
    let conn = Connection::open("pokemondata.db")?;
       
    // Prepare and execute a query
    let mut stmt = conn.prepare("SELECT * FROM pokemon WHERE name = ?")?;
    let mut pokemon_iter = stmt.query_map(&[name], |row| {
        Ok(Pokemon {
            name: row.get(0)?,
            id: row.get(1)?,
            type1: row.get(2)?,
            type2: row.get(3)?,
            base_hp: row.get(4)?,
            base_attack: row.get(5)?,
            base_defense: row.get(6)?,
            base_sp_attack: row.get(7)?,
            base_sp_defense: row.get(8)?,
            base_speed: row.get(9)?,
            ability: row.get(10)?,
            moveset: None,
            total_hp: 0
        })
    })?;
    
    // Fetch the result and assign it to a variable
    //let poke = pokemon_iter.next().unwrap()?;
    //println!("{:?}", poke);

    if let Some(result) = pokemon_iter.next() {
        return result;
    }
    return Err(rusqlite::Error::QueryReturnedNoRows);
}



fn get_moves (name: &String) -> Result<Move, rusqlite::Error> {
    let conn = Connection::open("pokemondata.db")?;
     
    // Prepare and execute a query
    let mut stmt = conn.prepare("SELECT * FROM moves WHERE name = ?")?;
    let mut move_iter = stmt.query_map(&[name], |row| {
        Ok(Move {
            name: row.get(0)?,
            type1: row.get(1)?,
            base_power: row.get(2)?,
            accuracy: row.get(3)?,
            category: row.get(4)?,
            priority: row.get(5)?,
            pp: row.get(6)?,
            crit: row.get(7)?,
            description: row.get(8)?,
        })
    })?;
    
    if let Some(result) = move_iter.next() {
        return result;
    }
    return Err(rusqlite::Error::QueryReturnedNoRows);

}