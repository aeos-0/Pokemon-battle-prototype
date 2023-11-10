use rand::{thread_rng, Rng};
use serde_json::json;
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

        //Have the user pick moves on both of the pokemon in play
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        input.make_ascii_lowercase();

        let move1 = get_moves(&input).unwrap();

        println!("");

        //Setup second pokemon
        println!("What will {:?} do?", poke2.name);

        //Display list of moves for the pokemon
        if let Some(move_vec) = &poke2.moveset {
            for pokemon_move in move_vec {
                println!("- {:?}", pokemon_move.name);     
            }
        } else {
            println!("{:?} has no moves and is unable to battle", poke2.name);
            std::process::exit(1);
        }

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        input.make_ascii_lowercase();
        
        let move2 = get_moves(&input).unwrap();

        //Now that we have both moves calculate speeds and damages
        //First priority then speeds
        //For priority only check if one is higher than the other
        if move1.priority > move2.priority { 
            //First mon moves first, ignore speed check
            damage_calc(&poke1, &move1, &mut poke2);
            damage_calc(&poke2, &move2, &mut poke1);
        } else if move2.priority > move1.priority {
            //Second mon moves ignore speed check
            damage_calc(&poke2, &move2, &mut poke1);
            damage_calc(&poke1, &move1, &mut poke2);
        }



        //If priority is the same then go off the speed stat
        if poke1.base_speed > poke2.base_speed { 
            damage_calc(&poke1, &move1, &mut poke2);
            damage_calc(&poke2, &move2, &mut poke1);
        } else if poke2.base_speed > poke1.base_speed {
            damage_calc(&poke2, &move2, &mut poke1);
            damage_calc(&poke1, &move1, &mut poke2);
        } else {
            //Speed tie
            let mut rng = thread_rng();
            let value = rng.gen_bool(1.0/2.0);

            //Randomly decides one
            if value {
                damage_calc(&poke1, &move1, &mut poke2);
                damage_calc(&poke2, &move2, &mut poke1);
            } else {
                damage_calc(&poke2, &move2, &mut poke1);
                damage_calc(&poke1, &move1, &mut poke2);
            }

        }
    }
    


}



fn damage_calc(attacker :&Pokemon, move_ :&Move, defender: &mut Pokemon) {
        //Setup json file
        let matchup: serde_json::Value = json!({
            "normal": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 0.5,
                "ghost": 0,
                "dragon": 1.0,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 1.0
            },
            "fire": {
                "normal": 1.0,
                "fire": 0.5,
                "water": 0.5,
                "grass": 2.0,
                "electric": 1.0,
                "ice": 2.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 2.0,
                "rock": 0.5,
                "ghost": 1.0,
                "dragon": 0.5,
                "dark": 1.0,
                "steel": 2.0,
                "fairy": 1.0
            },
            "water": {
                "normal": 1.0,
                "fire": 2.0,
                "water": 0.5,
                "grass": 0.5,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 2.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 2.0,
                "ghost": 1.0,
                "dragon": 0.5,
                "dark": 1.0,
                "steel": 1.0,
                "fairy": 1.0
            },
            "grass": {
                "normal": 1.0,
                "fire": 0.5,
                "water": 2.0,
                "grass": 0.5,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 0.5,
                "ground": 2.0,
                "flying": 0.5,
                "psychic": 1.0,
                "bug": 0.5,
                "rock": 2.0,
                "ghost": 1.0,
                "dragon": 0.5,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 1.0
            },
            "electric": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 2.0,
                "grass": 0.5,
                "electric": 0.5,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 0,
                "flying": 2.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 1.0,
                "dragon": 0.5,
                "dark": 1.0,
                "steel": 1.0,
                "fairy": 1.0
            },
            "ice": {
                "normal": 1.0,
                "fire": 0.5,
                "water": 0.5,
                "grass": 2.0,
                "electric": 1.0,
                "ice": 0.5,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 2.0,
                "flying": 2.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 1.0,
                "dragon": 2.0,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 1.0
            },
            "fighting": {
                "normal": 2.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 2.0,
                "fighting": 1.0,
                "poison": 0.5,
                "ground": 1.0,
                "flying": 0.5,
                "psychic": 0.5,
                "bug": 0.5,
                "rock": 2.0,
                "ghost": 0,
                "dragon": 1.0,
                "dark": 2.0,
                "steel": 2.0,
                "fairy": 0.5
            },
            "poison": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 2.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 0.5,
                "ground": 0.5,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 0.5,
                "ghost": 0.5,
                "dragon": 1.0,
                "dark": 1.0,
                "steel": 1.0,
                "fairy": 2.0
            },
            "ground": {
                "normal": 1.0,
                "fire": 2.0,
                "water": 1.0,
                "grass": 0.5,
                "electric": 2.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 2.0,
                "ground": 1.0,
                "flying": 0,
                "psychic": 1.0,
                "bug": 0.5,
                "rock": 2.0,
                "ghost": 1.0,
                "dragon": 1.0,
                "dark": 1.0,
                "steel": 2.0,
                "fairy": 1.0
            },
            "flying": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 2.0,
                "electric": 0.5,
                "ice": 1.0,
                "fighting": 2.0,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 2.0,
                "rock": 0.5,
                "ghost": 1.0,
                "dragon": 1.0,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 1.0
            },
            "psychic": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 2.0,
                "poison": 2.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 0.5,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 1.0,
                "dragon": 1.0,
                "dark": 0,
                "steel": 0.5,
                "fairy": 1.0
            },
            "bug": {
                "normal": 1.0,
                "fire": 0.5,
                "water": 1.0,
                "grass": 2.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 0.5,
                "poison": 0.5,
                "ground": 1.0,
                "flying": 0.5,
                "psychic": 2.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 0.5,
                "dragon": 1.0,
                "dark": 2.0,
                "steel": 0.5,
                "fairy": 0.5
            },
            "rock": {
                "normal": 1.0,
                "fire": 2.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 2.0,
                "fighting": 0.5,
                "poison": 1.0,
                "ground": 0.5,
                "flying": 2.0,
                "psychic": 1.0,
                "bug": 2.0,
                "rock": 1.0,
                "ghost": 1.0,
                "dragon": 1.0,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 1.0
            },
            "ghost": {
                "normal": 0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 2.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 2.0,
                "dragon": 1.0,
                "dark": 0.5,
                "steel": 1.0,
                "fairy": 1.0
            },
            "dragon": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 1.0,
                "dragon": 2.0,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 0
            },
            "dark": {
                "normal": 1.0,
                "fire": 1.0,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 0.5,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 2.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 2.0,
                "dragon": 1.0,
                "dark": 0.5,
                "steel": 1.0,
                "fairy": 0.5
            },
            "steel": {
                "normal": 1.0,
                "fire": 0.5,
                "water": 0.5,
                "grass": 1.0,
                "electric": 0.5,
                "ice": 2.0,
                "fighting": 1.0,
                "poison": 1.0,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 2.0,
                "ghost": 1.0,
                "dragon": 1.0,
                "dark": 1.0,
                "steel": 0.5,
                "fairy": 2.0
            },
            "fairy": {
                "normal": 1.0,
                "fire": 0.5,
                "water": 1.0,
                "grass": 1.0,
                "electric": 1.0,
                "ice": 1.0,
                "fighting": 2.0,
                "poison": 0.5,
                "ground": 1.0,
                "flying": 1.0,
                "psychic": 1.0,
                "bug": 1.0,
                "rock": 1.0,
                "ghost": 1.0,
                "dragon": 2.0,
                "dark": 2.0,
                "steel": 0.5,
                "fairy": 1.0
            }
        });
    println!("");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{:?} used {:?}!", attacker.name, move_.name);

    let mut rng = thread_rng();
    //Check if the move passes accuracy
    let accuracy = rng.gen_bool(move_.accuracy as f64 / 100.0);
    if !accuracy {
        //The move missed
        println!("{:?} avoided the attack!", &defender.name);
        return;
    }

    //Check for stab
    let stab;
    if attacker.type1 == move_.type1 || attacker.type2.as_ref().unwrap() == &move_.type1 {
        stab = 1.5;
    } else {
        stab = 1.0;
    }

    //Check for crit
    let critical;
    if rng.gen_bool(move_.crit as f64 / 100.0) { 
        critical = 1.5;
    } else {
        critical = 1.0;
    }

    //Random number for damage rolls   
    let random = rng.gen_range(85.0..=100.0) / 100.0;

    //Special or physical?
    let cat: f64;
    match move_.category.as_str() {
        "physical" => cat = attacker.base_attack as f64 / defender.base_defense as f64,
        "special" => cat = attacker.base_sp_attack as f64 / defender.base_sp_defense as f64,
        _ => panic!("Unexpected move category value"),
    }

    //Check type matchup
    let effectiveness;
    let effective_num;
    let mut effective_text = String::new();
    //Check if second type exists
    match defender.type2 {
        Some(_) => {
            // There are 2 types
            effectiveness = matchup[&move_.type1][&defender.type1].to_string();
            let float1: Result<f64, _> = effectiveness.parse();

            let float2: Result<f64, _> = (matchup[&move_.type1.to_string()][&defender.type2.as_ref().unwrap().to_string()].to_string()).parse();

            effective_num = float1.unwrap() * float2.unwrap();
            println!("Effective_num: {:?}", effective_num);
        }
        None => {
            // Only 1 type
            effectiveness = matchup[&move_.type1][&defender.type1].to_string();
            effective_num = effectiveness.parse().unwrap();
        }
    }

    //Add matchup text
    if effective_num == 0.0 {
        effective_text = "It doesn't have any effect...".to_string();
    } else if effective_num >= 2.0 {
        effective_text = "It's super effective!".to_string();
    } else {
        effective_text = "It's not very effective...".to_string();
    }

    //Actual calc
    let damage = ((22.0 * move_.base_power as f64 * cat / 50.0 + 2.0) * critical * random as f64 * stab * effective_num).round() as u32;

    //Effectiveness
    if !effective_text.is_empty() {
        println!("{0}", effective_text);
    }
    
    //Was it a crit?
    if critical == 1.5 {
        println!("A critical hit!");
    }

    println!("{:?} took {:?} damage.", &defender.name, damage);

    if damage >= defender.total_hp {
        //Pokemon is cooked
        println!("{:?} has fainted!", &defender.name);
        defender.total_hp = 0;
        println!("{:?} wins!", attacker.name);
        std::process::exit(0);
    }

    defender.total_hp -= damage;  

    if defender.total_hp == 0 {
        println!("{:?} has fainted!", &defender.name);
        println!("{:?} wins!", attacker.name);
        std::process::exit(0);
    }

    //Tell how much percent hp they have left remaining
    let percent = (defender.total_hp as f64 / defender.base_hp as f64 * 100.0).round() as u32;
    println!("{:?} has {:?} % hp remaining.", defender.name, percent);

    println!("");
}



//Sets up the pokemon stats based off of base values, neutral nature, and randomized ivs
fn get_stats(mut poke :Pokemon) -> Pokemon {
    //Generate random ivs every time
    let mut rng = thread_rng();

    //Calculate hp
    let mut iv = rng.gen_range(0..=31);
    let mut calc: u32 = ((2 * poke.base_hp + iv) * 50 /100) + 60;
    poke.base_hp = calc;
    poke.total_hp = calc;

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
    println!("");
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