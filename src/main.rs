use std::io::{self, Write};
use serde_json::Value as Json;

struct Weather {
    temp: String,
    windspeed: String,
}

#[tokio::main]
async fn main() {
    let latitude:f64 = input("Digite a latitude: ").trim().parse().unwrap();
    let longitude:f64 = input("Digite a longitude: ").trim().parse().unwrap();

    // PARA TESTES:
    // -3.72
    // -38.54
    
    let weather = get_weather(latitude, longitude).await;

    let current_weather = Weather {
        temp: weather["current_weather"]["temperature"].to_string(),
        windspeed: weather["current_weather"]["windspeed"].to_string(),
    };

    println!("Na latitude e longitude especificadas, a temperatura está em {temp}C e a velocidade do ar está em {ws} KM/H, aproximadamente.", temp=current_weather.temp, ws=current_weather.windspeed);
}


fn input(str: &str) -> String {
    let mut var = String::new();
    print!("{}", str);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut var)
        .expect("Falha ao ler a entrada");

    return var;
}

async fn get_weather(latitude: f64, longitude: f64) -> Json {
    let api_url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat:.2}&longitude={lon:.2}&current_weather=true", lat=latitude, lon=longitude);
    let result = reqwest::get(&api_url).await.unwrap().text_with_charset("utf-8").await.unwrap();
    let json:Json = serde_json::from_str(&result.as_str()).unwrap();
    return json;
}
