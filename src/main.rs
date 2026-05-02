use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    humidity: u32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    name: String,
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let city = match args.get(1) {
        None => "Algeria",
        Some(value) => value.as_str(),
    };

    let api_key = match std::env::var("OPENWEATHERMAP_APIKEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: OWM_API_KEY environment variable not set");
            eprintln!("Get a free key at https://openweathermap.org/api");
            eprintln!("Then run: export OWM_API_KEY=your_key_here");
            std::process::exit(1);
        }
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={api_key}"
    );
    let res = reqwest::blocking::get(url);
    match res {
        Err(err) => {
            eprintln!("ERROR: error fetching data {err}");
            return;
        }
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("Error: city '{}' not found or bad API key", city);
                return;
            }
            match resp.json::<WeatherResponse>() {
                Err(e) => eprintln!("Failed to parse response: {e}"),
                Ok(data) => {
                    // println!("name {name}", name = data.main.temp);
                    display_weather(&data);
                }
            }
        }
    }
}

fn display_weather(data: &WeatherResponse) {
    let description = data
        .weather
        .first()
        .map(|w| w.description.as_str())
        .unwrap_or("N/A");
    let icon = data
        .weather
        .first()
        .map(|w| w.icon.as_str())
        .unwrap_or("01d");

    println!("------------------------------");
    println!(" Weather in {}", data.name);
    println!("------------------------------");
    println!(" Condition  : {} {}", description, icon_to_emoji(icon));
    println!(" Temp       : {:.1}°C", data.main.temp);
    println!(" Feels like : {:.1}°C", data.main.feels_like);
    println!(" Humidity   : {}%", data.main.humidity);
    println!(" Wind speed : {:.1} m/s", data.wind.speed);
    println!("------------------------------");
}
fn icon_to_emoji(icon: &str) -> &str {
    match &icon[..2] {
        // first 2 chars = condition, 3rd = d/n
        "01" => "☀️ ", // clear sky
        "02" => "⛅ ", // few clouds
        "03" => "🌥️ ", // scattered clouds
        "04" => "☁️ ", // broken/overcast clouds
        "09" => "🌧️ ", // shower rain
        "10" => "🌦️ ", // rain
        "11" => "⛈️ ", // thunderstorm
        "13" => "❄️ ", // snow
        "50" => "🌫️ ", // mist/fog
        _ => "🌡️ ",    // fallback
    }
}
