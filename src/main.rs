use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use clap::{Parser, ValueEnum};

/*
* NOTE: 
* For now this will only update the config for window.opacity,
* that too intention is either it will increment or decrement the opacity.
* But in future i think we can have it update any property with type safety.
* */


/*
* - Increment/Decrement Window Opacity.
*   - From cmd args, accept operationType (increment/decrement)
*   - read the config file into config struct. Since the config has many more options
*     for, now our struct will have only with which we are concerned. To capture,
*     rest of the fields, we can use the serde's flatten feature.
*   - Once the config has been read into the struct, do some basic validations
*       - if property exists or not.
*       - if not, do we want to assume a default value.
*   - Once you have the current value, do the increment/decrement operation.
*
* */

#[derive(Debug, Serialize, Deserialize)]
struct WindowConfig {
    opacity : Option<f32>
}

#[derive(Debug, Serialize, Deserialize)]
struct AlacrittyConfig {
    #[serde(flatten)]
    others: HashMap<String,  Value>,
    window: Option<WindowConfig>,
}


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long = "opacity", allow_negative_numbers = true)]
    opacity: f32,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
    let config_path = "/home/manya/.config/alacritty/alacritty.yml";
    println!("{}", cli.opacity);

    let config_str;

     match fs::read_to_string(config_path) {
        Ok(cf) => { config_str = cf},
        Err(err) => {
            println!("error while reading config file: {}", err);
            return;
        }
    }

    println!("found config");
    let mut config;
    match serde_yaml::from_str::<AlacrittyConfig>(&config_str){
        Ok(cf) => { config = cf},
        Err(err) =>  {
            println!("error: {}", err);
            return;
        }
    }

    // println!("config parsed successfullly, config: {:?}", config);
    println!("config parsed successfullly");

    {
        let mut window = config.window.as_mut().unwrap();
        // let opacity = window.opacity.unwrap();
        window.opacity = Some(window.opacity.unwrap() + cli.opacity);
    }
    // println!("new config: {:?}", config);
    fs::write(config_path, serde_yaml::to_string(&config).unwrap()).unwrap();
}
