use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MeteoData {
    pub latitude: f32,
    pub longitude: f32,
    pub generationtime_ms: f32,
    pub utc_offset_seconds: f32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f32,
    pub current_weather_units: WeatherUnitsData,
    pub current_weather_interval_seconds: u32,
    pub current_weather: WeatherData,
    pub hourly_units: HourlyUnitsData,
    pub hourly: HourlyData,
    pub daily_units: DailyUnitsData,
    pub daily: DailyData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherUnitsData {
    pub time: String,
    pub temperature: String,
    pub windspeed: String,
    pub winddirection: String,
    pub is_day: String,
    pub weathercode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    pub time: String,
    pub temperature: f32,
    pub windspeed: f32,
    pub is_day: u8,
    pub weathercode: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourlyUnitsData {}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub relativehumidity_2m: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyUnitsData {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyData {}

pub async fn request_weather(url: &str) -> Result<MeteoData, Box<dyn std::error::Error>> {
    let result = reqwest::get(url).await?;
    let parsed = result.json::<MeteoData>().await?;
    Ok(parsed)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoData {
    pub lon: String,
    pub lat: String,
}

impl GeoData {
    pub fn new(longitude: String, latitude: String) -> Self {
        GeoData {
            lon: longitude,
            lat: latitude,
        }
    }

    pub fn update(&mut self, longitude: String, latitude: String) {
        self.lon = longitude;
        self.lat = latitude;
    }
}

pub fn request_geolocation(location: &str) -> Result<Vec<GeoData>, Box<dyn std::error::Error>> {
    let mut url = String::from("https://geocode.maps.co/search?q=");
    url.push_str(location);
    println!("{}", url);
    let result = reqwest::blocking::get(url)?;
    println!("{:?}", result);
    let parsed = result.json::<Vec<GeoData>>()?;
    println!("{:?}", parsed[0]);

    Ok(parsed)
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::*;

    #[test]
    fn current_weather() {
        let test_data = fs::read_to_string("test/weather_data.json").expect("Test file not found");
        let parsed_data: MeteoData = serde_json::from_str(&test_data).unwrap();

        assert_eq!(parsed_data.current_weather.temperature, 14.2);
        assert_eq!(parsed_data.current_weather.windspeed, 8.3);
    }

    #[test]
    fn hourly_weather() {
        let test_data = fs::read_to_string("test/weather_data.json").expect("Test file not found");
        let parsed_data: MeteoData = serde_json::from_str(&test_data).unwrap();

        assert_eq!(parsed_data.hourly.time[0], "2023-10-08T00:00");
        assert_eq!(parsed_data.hourly.temperature_2m[0], 11.6);
        assert_eq!(parsed_data.hourly.relativehumidity_2m[0], 83.0);
    }

    #[test]
    fn geo_location() {
        let test_data = fs::read_to_string("test/geo_data.json").expect("test file not found");
        // println!("{:?}", test_data);
        let parsed_data: Vec<GeoData> = serde_json::from_str(&test_data).unwrap();
        println!("{:?}", parsed_data);

        assert_eq!(parsed_data[0].lon, "7.7689951");
        assert_eq!(parsed_data[0].lat, "49.4432174");
    }
}
