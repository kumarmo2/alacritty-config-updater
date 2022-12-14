use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use clap::{Parser};

/*
* NOTE: 
* For now this will only update the config for window.opacity,
* that too intention is either it will increment or decrement the opacity.
* But in future i think we can have it update any property with type safety.
* */


/*
* - TODOs:
*   - get rid of all the unwraps and execpt calls which can panic.
*   - right now, once this program runs, the order of keys in the config files
*     changes. so if you check the git diff, it will show the whole file changed.
*     even though we are only updating the window.opacity value, but since we are not maintaining
*     the order of keys while writing back to the file, we are seeing this issue. IDEALLY, WE WOULD
*     WANT even the order to be same as before.
* */

#[derive(Debug, Serialize, Deserialize)]
struct WindowConfig {
    opacity : Option<f32>,
    #[serde(flatten)]
    others: HashMap<String, Value>
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
    let config_path = "/home/manya/.config/alacritty/alacritty.yml";

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
    {
        let mut window = config.window.as_mut().unwrap();
        // let opacity = window.opacity.unwrap();
        let mut new_opacity = window.opacity.unwrap() + cli.opacity;
        if new_opacity < 0.0 {
            new_opacity = 0.0;
        }
        if new_opacity > 1.0 {
            new_opacity = 1.0
        }

        window.opacity = Some(new_opacity);
    }
    fs::write(config_path, serde_yaml::to_string(&config).unwrap()).unwrap();
}
