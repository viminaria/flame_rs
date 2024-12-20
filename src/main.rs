use clap::{arg, command, value_parser, Arg, ArgAction};
use config::{Config, File, FileFormat};
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use random_choice::random_choice;
use rayon::prelude::*;
use std::{cmp::Ordering, env, io, path::PathBuf, sync::Mutex, time::Instant};
use thousands::Separable;

#[derive(Clone)]
struct Item<T> {
    n: &'static str,
    v: Vec<T>,
}

fn build_flame(
    stat: &str,
    option_table: Vec<Item<Item<u16>>>,
    flamestat: &str,
    noboss: bool,
    allstat: f32,
    substat: f32,
    att: f32,
    hpmp: f32,
) -> (Vec<(&'static str, u16)>, f32) {
    let tier_weights: Vec<Item<f32>> = vec![
        Item {
            n: "abyss",
            v: vec![0.0, 0.0, 0.0, 0.0, 0.63, 0.34, 0.03, 0.0, 0.0],
        },
        Item {
            n: "totem",
            v: vec![0.0, 0.0, 0.558, 0.325, 0.065, 0.032, 0.02, 0.0, 0.0],
        },
        Item {
            n: "drop",
            v: vec![0.0, 0.0, 0.25, 0.3, 0.3, 0.14, 0.01, 0.0, 0.0],
        },
        Item {
            n: "pflame",
            v: vec![0.0, 0.0, 0.2, 0.3, 0.36, 0.14, 0.0, 0.0, 0.0],
        },
        Item {
            n: "eflame",
            v: vec![0.0, 0.0, 0.0, 0.29, 0.45, 0.25, 0.01, 0.0, 0.0],
        },
        Item {
            n: "regcraft",
            v: vec![0.0, 0.0, 0.5, 0.4, 0.1, 0.0, 0.0, 0.0, 0.0],
        },
        Item {
            n: "mastercraft",
            v: vec![0.0, 0.0, 0.15, 0.3, 0.4, 0.14, 0.01, 0.0, 0.0],
        },
        Item {
            n: "meistercraft",
            v: vec![0.0, 0.0, 0.0, 0.19, 0.5, 0.3, 0.01, 0.0, 0.0],
        },
        Item {
            n: "masterfuse",
            v: vec![0.0, 0.0, 0.25, 0.35, 0.3, 0.1, 0.0, 0.0, 0.0],
        },
        Item {
            n: "meisterfuse",
            v: vec![0.0, 0.0, 0.0, 0.4, 0.45, 0.14, 0.01, 0.0, 0.0],
        },
    ]
    .into_iter()
    .filter(|a| a.n == flamestat)
    .collect();

    // return values
    let mut score = 0.0;
    let mut flame = vec![
        ("str", 0),
        ("dex", 0),
        ("int", 0),
        ("luk", 0),
        ("att", 0),
        ("matt", 0),
        ("hp", 0),
        ("mp", 0),
        ("jmp", 0),
        ("speed", 0),
        ("as", 0),
    ];

    // line weight values
    let mut weights = &tier_weights[0].v[..7];
    let mut lines = 4;

    // set noboss line weight values
    if noboss {
        weights = &tier_weights[0].v[2..];
        let a = [1, 2, 3, 4];
        let lineweights = [0.39, 0.39, 0.18, 0.04];
        let choice = random_choice().random_choice_f32(&a, &lineweights, 1);
        lines = *choice[0];
    }

    // build flame
    for option in option_table
        .choose_multiple(&mut rand::thread_rng(), lines)
        .into_iter()
    {
        // chooses 4 random flame_template from option table
        for choice in random_choice().random_choice_f32(&option.v[0].v, weights, 1) {
            // chooses tier by weight
            // add chosen line to flame
            if option.n == "str" {
                flame[0].1 += *choice;
            } else if option.n == "strdex" {
                flame[0].1 += *choice;
                flame[1].1 += *choice;
            } else if option.n == "strint" {
                flame[0].1 += *choice;
                flame[2].1 += *choice;
            } else if option.n == "strluk" {
                flame[0].1 += *choice;
                flame[3].1 += *choice;
            } else if option.n == "dex" {
                flame[1].1 += *choice;
            } else if option.n == "dexint" {
                flame[1].1 += *choice;
                flame[2].1 += *choice;
            } else if option.n == "dexluk" {
                flame[1].1 += *choice;
                flame[3].1 += *choice;
            } else if option.n == "int" {
                flame[2].1 += *choice;
            } else if option.n == "lukint" {
                flame[2].1 += *choice;
                flame[3].1 += *choice;
            } else if option.n == "luk" {
                flame[3].1 += *choice;
            } else if option.n == "att" {
                flame[4].1 += *choice;
            } else if option.n == "matt" {
                flame[5].1 += *choice;
            } else if option.n == "hp" {
                flame[6].1 += *choice;
            } else if option.n == "mp" {
                flame[7].1 += *choice;
            } else if option.n == "jmp" {
                flame[8].1 += *choice;
            } else if option.n == "speed" {
                flame[9].1 += *choice;
            } else if option.n == "as" {
                flame[10].1 += *choice;
            }
            // score flame
            if stat == "str" {
                score = flame[10].1 as f32 * allstat
                    + flame[4].1 as f32 * att
                    + flame[0].1 as f32
                    + flame[1].1 as f32 * substat;
            } else if stat == "luk" {
                score = flame[10].1 as f32 * allstat
                    + flame[4].1 as f32 * att
                    + flame[3].1 as f32
                    + flame[1].1 as f32 * substat;
            } else if stat == "dex" {
                score = flame[10].1 as f32 * allstat
                    + flame[4].1 as f32 * att
                    + flame[1].1 as f32
                    + flame[0].1 as f32 * substat;
            } else if stat == "int" {
                score = flame[10].1 as f32 * allstat
                    + flame[5].1 as f32 * att
                    + flame[2].1 as f32
                    + flame[3].1 as f32 * substat;
            } else if stat == "kanna" {
                score = flame[10].1 as f32 * allstat
                    + flame[5].1 as f32 * att
                    + flame[2].1 as f32
                    + flame[3].1 as f32 * substat
                    + flame[6].1 as f32 / hpmp
                    + flame[7].1 as f32 / hpmp
            } else if stat == "da" {
                score = flame[10].1 as f32 * allstat
                    + flame[4].1 as f32 * att
                    + flame[6].1 as f32
                    + flame[0].1 as f32 * substat
            } else if stat == "alt_thief" {
                score = flame[10].1 as f32 * allstat
                    + flame[4].1 as f32 * att
                    + flame[3].1 as f32
                    + flame[0].1 as f32 * substat
                    + flame[1].1 as f32 * substat
            } else if stat == "xenon" {
                score = flame[10].1 as f32 * allstat
                    + flame[4].1 as f32 * att
                    + flame[0].1 as f32
                    + flame[1].1 as f32
                    + flame[3].1 as f32
            }
        }
    }
    (flame, score) // add score return value
}

fn inner_main() -> io::Result<PathBuf> {
    let mut exe = env::current_exe()?;
    exe.set_file_name("flame_values.json");
    Ok(exe)
}

fn main() {
    let mut dir = String::new();

    match inner_main() {
        Ok(path) => dir = path.as_path().display().to_string(),
        Err(e) => {
            eprintln!("Error determining executable path: {}", e);
        }
    }

    let builder = Config::builder().add_source(File::new(&dir, FileFormat::Json));

    let mut allstat = 8.0;
    let mut allstat_x = 20.0; // xenon
    let mut substat = 0.10;
    let mut att = 3.0;
    let mut att_d = 20.0; // da
    let mut att_x = 6.0; // xenon
    let mut hpmp = 120.0; // kanna

    match builder.build() {
        Ok(config) => {
            allstat = config.get_float("allstat").unwrap() as f32;
            allstat_x = config.get_float("allstat_x").unwrap() as f32;
            substat = config.get_float("substat").unwrap() as f32;
            att = config.get_float("att").unwrap() as f32;
            att_d = config.get_float("att_d").unwrap() as f32;
            att_x = config.get_float("att_x").unwrap() as f32;
            hpmp = config.get_float("hpmp").unwrap() as f32;
        }
        Err(e) => {
            println!("error parsing: {:?}", e);
        }
    }

    let matches = command!()
    .arg(
        arg!(
            -t --trials <TRIALS> "Amount of times to run the simulator"
        )
        .value_parser(value_parser!(u64))
        .default_value("100000"),
    )
    .arg(
        arg!(
            -s --stat <STAT> "Stat to roll for [options: str, dex, int, luk, kanna, da, xenon, alt_thief]"
        )
        .value_parser(value_parser!(String))
        .default_value("str"),
    )
    .arg(
        arg!(
            -l --level <LEVEL> "Equip level [options: 100-109, 110-119, 120-129, 130-139, 140-149, 150-159, 160-169, 170-179, 180-189, 190-199, 200-249, 250+]"
        )
        .value_parser(value_parser!(String))
        .default_value("140-149"),
    )
    .arg(
        arg!(
            -k --keep <THRESHOLD> "Minimum flamescore target"
        )
        .value_parser(value_parser!(f32))
        .default_value("100"),
    )
    .arg(
        arg!(
            -f --flametype <FLAMETYPE> "Type of flame used [options: abyss, totem, drop, pflame, eflame, regcraft, mastercraft, meistercraft, masterfuse, meisterfuse]"
        )
        .value_parser(value_parser!(String))
        .default_value("pflame"),
    )
    .arg(
        arg!(
            --top <NUMBER> "Displays the top scoring flames (max 1000)"
        )
        .value_parser(value_parser!(usize))
        .required(false)
    )
    .arg(
        arg!(
            -c --chance <NUMBER> "Calculates the odds of getting target flame within the specified amount of flames"
        )
        .value_parser(value_parser!(u64))
        .required(false)
    )
    .arg(
        Arg::new("noboss")
            .short('n')
            .long("noboss")
            .help("Simulate non-boss flames")
            .action(ArgAction::SetTrue),
    )
    .get_matches();

    let trials = matches.get_one::<u64>("trials").unwrap();
    let stat = matches.get_one::<String>("stat").unwrap().to_owned();
    let keep = matches.get_one::<f32>("keep").unwrap();
    let level = matches.get_one::<String>("level").unwrap();
    let flametype = matches.get_one::<String>("flametype").unwrap();

    if stat == "da" {
        att = att_d;
    } else if stat == "xenon" {
        att = att_x;
        allstat = allstat_x;
    }

    let mut top: usize = 1;
    if let Some(tops) = matches.get_one::<usize>("top") {
        if *tops > 1000 {
            top = 1000;
        } else if *tops > *trials as usize {
            top = *trials as usize;
        } else {
            top = *tops;
        }
    }

    let mut chance: i32 = 0;
    if let Some(budget) = matches.get_one::<i32>("chance") {
        chance = *budget;
    }

    let mut noboss = false;
    if matches.get_flag("noboss") {
        noboss = matches.get_flag("noboss");
    }

    let now = Instant::now();

    let flame_collection: Mutex<Vec<(Vec<(&str, u16)>, f32)>> = Mutex::new(Vec::new());
    let count = Mutex::new(0);

    let flat_options: Vec<Item<u16>> = vec![
        Item {
            n: "100-109",
            v: vec![6, 12, 18, 24, 30, 36, 42],
        },
        Item {
            n: "110-119",
            v: vec![6, 12, 18, 24, 30, 36, 42],
        },
        Item {
            n: "120-129",
            v: vec![7, 14, 21, 28, 35, 42, 49],
        },
        Item {
            n: "130-139",
            v: vec![7, 14, 21, 28, 35, 42, 49],
        },
        Item {
            n: "140-149",
            v: vec![8, 16, 24, 32, 40, 48, 56],
        },
        Item {
            n: "150-159",
            v: vec![8, 16, 24, 32, 40, 48, 56],
        },
        Item {
            n: "160-169",
            v: vec![9, 18, 27, 36, 45, 54, 63],
        },
        Item {
            n: "170-179",
            v: vec![9, 18, 27, 36, 45, 54, 63],
        },
        Item {
            n: "180-189",
            v: vec![10, 20, 30, 40, 50, 60, 70],
        },
        Item {
            n: "190-199",
            v: vec![10, 20, 30, 40, 50, 60, 70],
        },
        Item {
            n: "200-249",
            v: vec![11, 22, 33, 44, 55, 66, 77],
        },
        Item {
            n: "250+",
            v: vec![12, 24, 36, 48, 60, 72, 84],
        },
    ]
    .into_par_iter()
    .filter(|a| a.n == level)
    .collect();
    let combo_options: Vec<Item<u16>> = vec![
        Item {
            n: "100-109",
            v: vec![3, 6, 9, 12, 15, 18, 21],
        },
        Item {
            n: "110-119",
            v: vec![3, 6, 9, 12, 15, 18, 21],
        },
        Item {
            n: "120-129",
            v: vec![4, 8, 12, 16, 20, 24, 28],
        },
        Item {
            n: "130-139",
            v: vec![4, 8, 12, 16, 20, 24, 28],
        },
        Item {
            n: "140-149",
            v: vec![4, 8, 12, 16, 20, 24, 28],
        },
        Item {
            n: "150-159",
            v: vec![4, 8, 12, 16, 20, 24, 28],
        },
        Item {
            n: "160-169",
            v: vec![5, 10, 15, 20, 25, 30, 35],
        },
        Item {
            n: "170-179",
            v: vec![5, 10, 15, 20, 25, 30, 35],
        },
        Item {
            n: "180-189",
            v: vec![5, 10, 15, 20, 25, 30, 35],
        },
        Item {
            n: "190-199",
            v: vec![5, 10, 15, 20, 25, 30, 35],
        },
        Item {
            n: "200-249",
            v: vec![6, 12, 18, 24, 30, 36, 42],
        },
        Item {
            n: "250+",
            v: vec![7, 14, 21, 28, 35, 42, 49],
        },
    ]
    .into_par_iter()
    .filter(|a| a.n == level)
    .collect();
    let hpmp_options: Vec<Item<u16>> = vec![
        Item {
            n: "100-109",
            v: vec![300, 600, 900, 1200, 1500, 1800, 2100],
        },
        Item {
            n: "110-119",
            v: vec![330, 660, 990, 1320, 1650, 1980, 2310],
        },
        Item {
            n: "120-129",
            v: vec![360, 720, 1080, 1440, 1800, 2160, 2520],
        },
        Item {
            n: "130-139",
            v: vec![390, 780, 1170, 1560, 1950, 2340, 2730],
        },
        Item {
            n: "140-149",
            v: vec![420, 840, 1260, 1680, 2100, 2520, 2940],
        },
        Item {
            n: "150-159",
            v: vec![450, 900, 1350, 1800, 2250, 2700, 3150],
        },
        Item {
            n: "160-169",
            v: vec![480, 960, 1440, 1920, 2400, 2880, 3360],
        },
        Item {
            n: "170-179",
            v: vec![510, 1020, 1530, 2040, 2550, 3060, 3570],
        },
        Item {
            n: "180-189",
            v: vec![540, 1080, 1620, 2160, 2700, 3240, 3780],
        },
        Item {
            n: "190-199",
            v: vec![570, 1140, 1710, 2280, 2850, 3420, 3990],
        },
        Item {
            n: "200-249",
            v: vec![600, 1200, 1800, 2400, 3000, 3600, 4200],
        },
        Item {
            n: "250+",
            v: vec![700, 1400, 2100, 2800, 3500, 4200, 4900],
        },
    ]
    .into_par_iter()
    .filter(|a| a.n == level)
    .collect();
    let basic_options: Vec<Item<u16>> = vec![
        Item {
            n: "100-109",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "110-119",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "120-129",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "130-139",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "140-149",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "150-159",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "160-169",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "170-179",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "180-189",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "190-199",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "200-249",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
        Item {
            n: "250+",
            v: vec![1, 2, 3, 4, 5, 6, 7],
        },
    ]
    .into_par_iter()
    .filter(|a| a.n == level)
    .collect();
    let option_table: Vec<Item<Item<u16>>> = vec![
        Item {
            n: "str",
            v: flat_options.to_vec(),
        },
        Item {
            n: "dex",
            v: flat_options.to_vec(),
        },
        Item {
            n: "int",
            v: flat_options.to_vec(),
        },
        Item {
            n: "luk",
            v: flat_options.to_vec(),
        },
        Item {
            n: "strdex",
            v: combo_options.to_vec(),
        },
        Item {
            n: "strint",
            v: combo_options.to_vec(),
        },
        Item {
            n: "strluk",
            v: combo_options.to_vec(),
        },
        Item {
            n: "dexint",
            v: combo_options.to_vec(),
        },
        Item {
            n: "dexluk",
            v: combo_options.to_vec(),
        },
        Item {
            n: "lukint",
            v: combo_options.to_vec(),
        },
        Item {
            n: "hp",
            v: hpmp_options.to_vec(),
        },
        Item {
            n: "mp",
            v: hpmp_options.to_vec(),
        },
        Item {
            n: "def",
            v: basic_options.to_vec(),
        },
        Item {
            n: "lvlred",
            v: basic_options.to_vec(),
        },
        Item {
            n: "att",
            v: basic_options.to_vec(),
        },
        Item {
            n: "matt",
            v: basic_options.to_vec(),
        },
        Item {
            n: "spd",
            v: basic_options.to_vec(),
        },
        Item {
            n: "jmp",
            v: basic_options.to_vec(),
        },
        Item {
            n: "as",
            v: basic_options.to_vec(),
        },
    ];

    let bar = ProgressBar::new(*trials);

    (0..*trials).into_par_iter().for_each(|_| {
        bar.inc(1);
        let flame = build_flame(
            &stat,
            option_table.clone(),
            flametype,
            noboss,
            allstat,
            substat,
            att,
            hpmp,
        );

        if flame.1 >= *keep {
            *count.lock().unwrap() += 1;
        }

        if flame_collection.lock().unwrap().len() < top {
            flame_collection.lock().unwrap().push(flame.clone());
            flame_collection.lock().unwrap().sort_by(
                |a: &(Vec<(&str, u16)>, f32), b: &(Vec<(&str, u16)>, f32)| {
                    // reverse sort collection by score
                    if a.1 < b.1 {
                        Ordering::Greater
                    } else if a.1 == b.1 {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                },
            );
        } else if flame.1 > flame_collection.lock().unwrap()[top - 1].1 {
            flame_collection.lock().unwrap().push(flame.clone());
            flame_collection.lock().unwrap().sort_by(
                |a: &(Vec<(&str, u16)>, f32), b: &(Vec<(&str, u16)>, f32)| {
                    // reverse sort collection by score
                    if a.1 < b.1 {
                        Ordering::Greater
                    } else if a.1 == b.1 {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                },
            );
            flame_collection.lock().unwrap().truncate(top);
        }
    });

    bar.finish();

    let mut average_flames: f32 = 0.0;

    if *count.lock().unwrap() > 0 {
        average_flames = *trials as f32 / *count.lock().unwrap() as f32;
    }

    println!(
        "Settings - Trials: {}, Flametype: {}, Stat: {}, Level: {} \n",
        trials.separate_with_commas(),
        flametype,
        stat,
        level
    );
    if noboss {
        println!("Noboss: {}", noboss)
    }
    println!("Results:");
    println!(
        "Flames over {} flamescore: {}/{} \n",
        *keep,
        count.lock().unwrap().clone().separate_with_commas(),
        trials.separate_with_commas()
    );
    println!(
        "Average flames: {}",
        (average_flames.ceil() as u32).separate_with_commas()
    );
    if flametype == "pflame" {
        if average_flames.ceil() * 0.00912 >= 1000.0 {
            println!(
                "Average cost: {:.5}T",
                (average_flames.ceil() * 0.00000912).separate_with_commas()
            );
        } else {
            println!(
                "Average cost: {:.5}b",
                (average_flames.ceil() * 0.00912).separate_with_commas()
            );
        }
    }
    println!("\n");
    if chance > 0 {
        let odds: f32 = 1.0 - ((1.0 - *count.lock().unwrap() as f32 / *trials as f32).powi(chance));
        println!(
            "Chance of getting within {} flames: {:.3}% \n",
            &chance,
            odds * 100.0
        );
    }

    if top > 1 {
        println!("Top {} flames:", top);
        let mut number = 1;
        for flame in flame_collection.lock().unwrap().clone().iter() {
            println!("#{}: {:?} with score: {:.2} \n", number, flame.0, flame.1);
            number += 1;
        }
    } else {
        for flame in flame_collection.lock().unwrap().clone().iter() {
            println!("Best flame:");
            for el in flame.0.iter() {
                if el.1 > 0 {
                    println!("{:?}, {:?}", el.0, el.1);
                }
            }
            println!("\nscore: {:.2} \n", flame.1);
        }
    }
    let elapsed = now.elapsed();
    println!("time: {:.3?}", elapsed);
}
