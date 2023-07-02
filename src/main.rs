
use std::fs;
use std::error::Error;
use clap::Parser;

// lat long are fixed (Dublin Ireland)
const LATITUDE: f64 = 53.286572;
const LONGITUDE: f64 = -6.370580;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("today"))]
    method: String,
    #[arg(short, long, default_value_t = 1)]
    days: u8
}


// Read api key from a file
fn get_api_key() -> String {
    
    let contents = fs::read_to_string("./api_key.txt")
        .expect("Issue reading API key");

    contents
}

fn main() -> Result<(), Box<dyn Error>>{

    // Process user arguments
    let args = Args::parse();
    let api_key = get_api_key();

    let url_string_current: String;

    //println!("{}", args.method);
    //println!("{}", args.days);


    match args.method {
        t if t.to_lowercase() == "today" => {
            url_string_current = format!("https://api.weatherapi.com/v1/current.json?key={api_key}&q={lat} {long}&aqi=yes", api_key=api_key, lat=LATITUDE, long=LONGITUDE);
        },
        t if t.to_lowercase() == "forecast" => {
            url_string_current= format!("https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={lat} {long}&days={days}&aqi=yes&alerts=no",api_key = api_key, lat=LATITUDE, long=LONGITUDE, days = args.days);
        },
        _ => { panic!("Program only accepts 'forecast' or 'today' as arguments") }
    };
    
    let body = reqwest::blocking::get(url_string_current)?.text()?;

    println!("{}", body);
    Ok(())
}
