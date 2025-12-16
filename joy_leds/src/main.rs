use std::{fs::{read_dir, File, read_to_string}, collections::HashMap, io::{Write, Result}, path::PathBuf};
use hidapi::HidApi;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    red: u8,
    green: u8,
    blue:   u8,
    global: u8,
}

const CONFIG: &'static str = "/etc/joyled.conf";

fn _device_present() -> bool {
    let mut present: bool = false;
    match HidApi::new() {
	Ok(api) => {
	    for device in api.device_list() {
		let i = format!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
		if "054c:05c4" == i {
		    present = true;
		} else {
		    present = false;
		}
	    }
	},
	Err(e) => {
	    eprintln!("Error: {e}");
	},
    }
    present
}

fn get_leds() -> HashMap<&'static str, PathBuf> {
    
    let mut present: bool = false;
    let mut leds = HashMap::new();
    
    match HidApi::new() {
	Ok(api) => {
	    for device in api.device_list() {
		let i = format!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
		if "054c:05c4" == i {
		    present = true;
		} else {
		    present = false;
		}
	    }
	},
	Err(e) => {
	    eprintln!("Error: {e}");
	},
    }

    if present {
	if let Ok(list) = read_dir("/sys/class/leds/") {
	    for color in list {
		if let Ok(color) = color {
		    if color.file_name().to_string_lossy().contains("red")
		    { leds.entry("red").or_insert(color.path().to_path_buf().join("brightness"));
		    } else if color.file_name().to_string_lossy().contains("green")
		    { leds.entry("green").or_insert(color.path().to_path_buf().join("brightness"));
		    } else if color.file_name().to_string_lossy().contains("blue")
		    { leds.entry("blue").or_insert(color.path().to_path_buf().join("brightness"));
		    } else if color.file_name().to_string_lossy().contains("global")
		    { leds.entry("global").or_insert(color.path().to_path_buf().join("brightness"));
		    } else { continue }
		}
	    }
	}
    }
    leds
}

fn read_config (input: &'static str) -> Config {

    let file: Vec<String> = read_to_string(input)
        .ok()
	.expect("Config not found.")
	.lines()
	.map(String::from)
	.collect();
    
    let config = Config {
	red: file[0].replace("red: ", "").parse().ok().expect("Red not found"),
	green: file[1].replace("green: ", "").parse().ok().expect("Green not found"),
	blue: file[2].replace("blue: ", "").parse().ok().expect("Blue not found"),
	global: file[3].replace("global: ", "").parse().ok().expect("Global not found"),
    };

    config

}

fn write_to_file(input: String, s: u8) -> Result<()>{
    
    let mut buff = File::create(input.to_string())?;
    buff.write(s.to_string().as_bytes())?;
    Ok(())

}

fn main() {
    
    let leds = get_leds();

    let red = leds.get_key_value("red");
    let green = leds.get_key_value("green");
    let blue = leds.get_key_value("blue");
    let global = leds.get_key_value("global");

    
    let _ = write_to_file(red.unwrap().1.to_str().unwrap().to_string(), read_config(CONFIG).red);
    let _ = write_to_file(green.unwrap().1.to_str().unwrap().to_string(), read_config(CONFIG).green);
    let _ = write_to_file(blue.unwrap().1.to_str().unwrap().to_string(), read_config(CONFIG).blue);
    let _ = write_to_file(global.unwrap().1.to_str().unwrap().to_string(), read_config(CONFIG).global);
    
    //    }
    
    //let mut pat = std::path::PathBuf::new();

    //pat.push(_test.get_key_value("red").unwrap().1.path());
    //pat.push(r"brightness");

    //println!("{:?}", _device_present());
    
    //println!("{:#?}", read_config(CONFIG));

    //let _ = write_to_file("test".to_string(), read_config(CONFIG).blue);
    
    println!("Still works!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_leds() {
	let leds = get_leds();
	println!("{:?}", leds.get_key_value("red:"));
	println!("{:?}", leds.get_key_value("green:"));
	println!("{:?}", leds.get_key_value("blue:"));
	println!("{:?}", leds.get_key_value("global:"));
    }

    #[test]
    fn device_is_present() {
	if _device_present() == true {
	    println!("Device is present")
	} else {
	    println!("Device not present")
	}
    }
}
