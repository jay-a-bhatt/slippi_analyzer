use peppi::game::Player;
use peppi::game::immutable::Game;
use peppi::io::slippi::read;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use walkdir::WalkDir;

pub const SLIPPI_PATH: &str = "slp/";

pub enum Sort {
    AlphabeticalAsc,
    AlphabeticalDesc,
    CountAsc,
    CountDesc,
}

pub enum Id {
    Code(String),
    Nickname(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Character {
    CaptainFalcon,
    DonkeyKong,
    Fox,
    MrGameAndWatch,
    Kirby,
    Bowser,
    Link,
    Luigi,
    Mario,
    Marth,
    Mewtwo,
    Ness,
    Peach,
    Pikachu,
    IceClimbers,
    Jigglypuff,
    Samus,
    Yoshi,
    Zelda,
    Sheik,
    Falco,
    YoungLink,
    DrMario,
    Roy,
    Pichu,
    Ganondorf,
    MasterHand,
    MaleWireframe,
    FemaleWireframe,
    GigaBowser,
    CrazyHand,
    Sandbag,
    Popo,
    Unknown,
}

impl Character {
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Self::CaptainFalcon,
            1 => Self::DonkeyKong,
            2 => Self::Fox,
            3 => Self::MrGameAndWatch,
            4 => Self::Kirby,
            5 => Self::Bowser,
            6 => Self::Link,
            7 => Self::Luigi,
            8 => Self::Mario,
            9 => Self::Marth,
            10 => Self::Mewtwo,
            11 => Self::Ness,
            12 => Self::Peach,
            13 => Self::Pikachu,
            14 => Self::IceClimbers,
            15 => Self::Jigglypuff,
            16 => Self::Samus,
            17 => Self::Yoshi,
            18 => Self::Zelda,
            19 => Self::Sheik,
            20 => Self::Falco,
            21 => Self::YoungLink,
            22 => Self::DrMario,
            23 => Self::Roy,
            24 => Self::Pichu,
            25 => Self::Ganondorf,
            26 => Self::MasterHand,
            27 => Self::MaleWireframe,
            28 => Self::FemaleWireframe,
            29 => Self::GigaBowser,
            30 => Self::CrazyHand,
            31 => Self::Sandbag,
            32 => Self::Popo,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Character::CaptainFalcon => "Captain Falcon",
            Character::DonkeyKong => "Donkey Kong",
            Character::Fox => "Fox",
            Character::MrGameAndWatch => "Mr. Game and Watch",
            Character::Kirby => "Kirby",
            Character::Bowser => "Bowser",
            Character::Link => "Link",
            Character::Luigi => "Luigi",
            Character::Mario => "Mario",
            Character::Marth => "Marth",
            Character::Mewtwo => "Mewtwo",
            Character::Ness => "Ness",
            Character::Peach => "Peach",
            Character::Pikachu => "Pikachu",
            Character::IceClimbers => "Ice Climbers",
            Character::Jigglypuff => "Jigglypuff",
            Character::Samus => "Samus",
            Character::Yoshi => "Yoshi",
            Character::Zelda => "Zelda",
            Character::Sheik => "Sheik",
            Character::Falco => "Falco",
            Character::YoungLink => "Young Link",
            Character::DrMario => "Dr. Mario",
            Character::Roy => "Roy",
            Character::Pichu => "Pichu",
            Character::Ganondorf => "Ganondorf",
            Character::MasterHand => "Master Hand",
            Character::MaleWireframe => "Male Wireframe",
            Character::FemaleWireframe => "Female Wireframe",
            Character::GigaBowser => "Giga Bowser",
            Character::CrazyHand => "Crazy Hand",
            Character::Sandbag => "Sandbag",
            Character::Popo => "Popo",
            Character::Unknown => "UNKNOWN",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    FountainOfDreams,
    PokemonStadium,
    PeachCastle,
    KongoJungle,
    Brinstar,
    Corneria,
    YoshisStory,
    Onett,
    MuteCity,
    RainbowCruise,
    JungleJapes,
    GreatBay,
    HyruleTemple,
    BrinstarDepths,
    YoshisIsland,
    GreenGreens,
    Fourside,
    MushroomKingdomI,
    MushroomKingdomII,
    Venom,
    PokeFloats,
    BigBlue,
    IcicleMountain,
    Icetop,
    FlatZone,
    DreamLandN64,
    YoshisIslandN64,
    KongoJungleN64,
    Battlefield,
    FinalDestination,
    NotStage,
}

impl Stage {
    pub fn from_id(id: usize) -> Self {
        match id {
            2 => Stage::FountainOfDreams,
            3 => Stage::PokemonStadium,
            4 => Stage::PeachCastle,
            5 => Stage::KongoJungle,
            6 => Stage::Brinstar,
            7 => Stage::Corneria,
            8 => Stage::YoshisStory,
            9 => Stage::Onett,
            10 => Stage::MuteCity,
            11 => Stage::RainbowCruise,
            12 => Stage::JungleJapes,
            13 => Stage::GreatBay,
            14 => Stage::HyruleTemple,
            15 => Stage::BrinstarDepths,
            16 => Stage::YoshisIsland,
            17 => Stage::GreenGreens,
            18 => Stage::Fourside,
            19 => Stage::MushroomKingdomI,
            20 => Stage::MushroomKingdomII,
            22 => Stage::Venom,
            23 => Stage::PokeFloats,
            24 => Stage::BigBlue,
            25 => Stage::IcicleMountain,
            26 => Stage::Icetop,
            27 => Stage::FlatZone,
            28 => Stage::DreamLandN64,
            29 => Stage::YoshisIslandN64,
            30 => Stage::KongoJungleN64,
            31 => Stage::Battlefield,
            32 => Stage::FinalDestination,
            _ => Stage::NotStage,
        }
    }
}

fn load_game(path: &str) -> Result<Game, Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let game =
        read(&mut reader, None).map_err(|_| format!("Failed to deserialize game from {}", path))?;

    return Ok(game);
}

pub fn scan_dir(path: &str) -> Vec<Game> {
    WalkDir::new(path)
        .into_iter()
        .par_bridge() 
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext == "slp")
        })
        .filter_map(|entry| {
            let path_str = entry.path().to_str()?;
            match load_game(path_str) {
                Ok(game) => Some(game),
                Err(e) => {
                    eprintln!("Failed to load game from {:?}: {}", entry.path(), e);
                    None
                }
            }
        })
        .collect()
}

fn get_player_from_port<'a>(game: &'a Game, port: usize) -> Option<&'a Player> {
    let index = match port {
        0..=3 => port,
        _ => return None,
    };
    game.start.players.get(index)
}

fn get_winner(game: &Game) -> Option<&Player> {
    let winner_port = game
        .end
        .as_ref()
        .and_then(|end| {
            end.players
                .as_ref()
                .and_then(|players| players.iter().find(|player_end| player_end.placement == 0))
        })
        .map(|player_end| player_end.port);

    return winner_port.and_then(|port| get_player_from_port(game, port as usize));
}

fn get_total_wins(games: &Vec<Game>, id: &Id) -> i32 {
    return games
        .par_iter()
        .filter_map(|game| {
            let winner = get_winner(game)?;
            let netplay = winner.netplay.as_ref()?;

            match &id {
                Id::Code(code) if &netplay.code.to_normalized() == code => Some(1),
                Id::Nickname(name) if &netplay.name.to_normalized() == name => Some(1),
                _ => None,
            }
        })
        .sum();
}

pub fn get_total_wins_per_character(games: &Vec<Game>, id: &Id, sort: Sort) -> String {
    let wins_by_character = Mutex::new(HashMap::new());

    games.par_iter().for_each(|game| {
        if let Some(winner) = get_winner(game) {
            if let Some(netplay) = &winner.netplay {
                let matched = match id {
                    Id::Code(code) => &netplay.code.to_normalized() == code,
                    Id::Nickname(name) => &netplay.name.to_normalized() == name,
                };

                if matched {
                    let character = Character::from_id(winner.character);
                    let mut map = wins_by_character.lock().unwrap();
                    *map.entry(character).or_insert(0) += 1;
                }
            }
        }
    });

    let mut sorted: Vec<_> = wins_by_character
        .into_inner()
        .unwrap()
        .into_iter()
        .collect();

    match sort {
        Sort::AlphabeticalAsc => sorted.sort_by_key(|(c, _)| c.to_string()),
        Sort::AlphabeticalDesc => sorted.sort_by_key(|(c, _)| std::cmp::Reverse(c.to_string())),
        Sort::CountAsc => sorted.sort_by_key(|(_, count)| *count),
        Sort::CountDesc => sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count)),
    }

    return sorted
        .into_iter()
        .map(|(character, count)| format!("{}: {}", character, count))
        .collect::<Vec<_>>()
        .join("\n");
}

fn main() {
    let game: Game = match load_game("slp/main1.slp") {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Failed to load game: {}", e);
            return;
        }
    };

    // Single Game Analysis
    if let Some(winner) = get_winner(&game) {
        // Get Name
        if let Some(netplay) = &winner.netplay {
            println!("{}", netplay.name.to_normalized())
        } else {
            println!("Winner has no netplay info.")
        }

        // Get Player Character
        let winner_character = Character::from_id(winner.character);
        println!("Winner character: {}", winner_character)

    } else {
        println!("No winner found")
    };

    // Bulk Analysis
    let identity: Id = Id::Code(String::from("JAY#909"));
    let games: Vec<Game> = scan_dir(SLIPPI_PATH);
    let total_wins: i32 = get_total_wins(&games, &identity);
    println!("Total wins = {}", total_wins);
    let total_wins_sorted = get_total_wins_per_character(&games, &identity, Sort::AlphabeticalAsc);
    print!("Total wins (Sorted per character) = \n{}", total_wins_sorted);
}
