
use std::fs;
use std::error::Error;
use clap::Parser;
use serde_json::Value;

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


fn generate_report(root: &serde_json::Value, report_type: &str, number_of_days: &str) -> String {

    let mut template: String = String::new();
    match report_type {
        val if val.to_lowercase() == "today" => {

            template = format!("
            [Location]: {l}\n
            [Date]: {t}\n
            [Summary]: {h}\n 
            [Temperature Deg/C]: {temp}\n
            [Rain mm]: {rain}\n
            [Wind km/hr]: {wind}
            ", 
            l = root["location"]["name"].to_string() + ", " 
                + &root["location"]["region"].to_string() + ", "
                + &root["location"]["country"].to_string(),
            t = root["current"]["last_updated"],
            h = root["current"]["condition"]["text"],
            temp = root["current"]["temp_c"],
            rain = root["current"]["precip_mm"],
            wind = root["current"]["wind_kph"]
        );

        },
        val if val.to_lowercase() == "forecast" => {

            let number_of_days = number_of_days;
            let mut counter = 0;

            loop 
            {
                if number_of_days == counter.to_string() {
                    break;
                }
                let day = counter + 1;
                let forecast_string = format!("
                --------------------------------
                [Forecast Day]: {day}\n
                [Location]: {l}\n
                [Date]: {t}\n
                [Summary]: {h}\n 
                [Temperature Deg/C]: {temp}\n
                [Rain mm]: {rain}\n
                [Wind km/hr]: {wind}
                --------------------------------
                \n
                ",
                day = day,
                l = root["location"]["name"].to_string() + ", " 
                    + &root["location"]["region"].to_string() + ", "
                    + &root["location"]["country"].to_string(),
                t = root["forecast"]["forecastday"][counter]["date"],
                h = root["forecast"]["forecastday"][counter]["day"]["condition"]["text"],
                temp = root["forecast"]["forecastday"][counter]["day"]["avgtemp_c"],
                rain = root["forecast"]["forecastday"][counter]["day"]["totalprecip_mm"],
                wind = root["forecast"]["forecastday"][counter]["day"]["maxwind_kph"]
                );
                template += &forecast_string;
                counter += 1;
            }
        },
        _ => {
            template = String::from("");
        }
    }

    //Extract relevant fields
    template
}


fn main() -> Result<(), Box<dyn Error>> {

    // Process user arguments
    let args = Args::parse();
    let api_key = get_api_key();

    let url_string_current: String;
    let report_type = args.method;
    let number_of_days = args.days.to_string();

    match &report_type {
        t if t.to_lowercase() == "today" => {
            url_string_current = format!("https://api.weatherapi.com/v1/current.json?key={api_key}&q={lat} {long}&aqi=yes", api_key=api_key, lat=LATITUDE, long=LONGITUDE);
        },
        t if t.to_lowercase() == "forecast" => {
            url_string_current= format!("https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={lat} {long}&days={days}&aqi=yes&alerts=no",api_key = api_key, lat=LATITUDE, long=LONGITUDE, days = number_of_days);
        },
        _ => { panic!("Program only accepts 'forecast' or 'today' as arguments") }
    };
    
    let body = reqwest::blocking::get(url_string_current)?.text()?;
    let root: Value = serde_json::from_str(&body)?;

    //Generate report for today
    println!("{}", generate_report(&root, &report_type, &number_of_days));
    Ok(())
}
