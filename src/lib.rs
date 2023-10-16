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
    pub current_units: WeatherUnitsData,
    pub current: WeatherData,
    // pub hourly_units: HourlyUnitsData,
    // pub hourly: HourlyData,
    // pub daily_units: DailyUnitsData,
    // pub daily: DailyData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherUnitsData {
    pub time: String,
    pub temperature_2m: String,
    pub relativehumidity_2m: String,
    pub apparent_temperature: String,
    pub is_day: String,
    pub precipitation: String,
    pub rain: String,
    pub showers: String,
    pub snowfall: String,
    pub weathercode: String,
    pub cloudcover: String,
    pub pressure_msl: String,
    pub surface_pressure: String,
    pub windspeed_10m: String,
    pub winddirection_10m: String,
    pub windgusts_10m: String,
    pub uv_index: String,
    pub uv_index_clear_sky: String,
    pub cape: String,
    pub freezinglevel_height: String,
    pub shortwave_radiation: String,
    pub direct_radiation: String,
    pub diffuse_radiation: String,
    pub direct_normal_irradiance: String,
    pub terrestrial_radiation: String,
    pub shortwave_radiation_instant: String,
    pub direct_radiation_instant: String,
    pub diffuse_radiation_instant: String,
    pub direct_normal_irradiance_instant: String,
    pub terrestrial_radiation_instant: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    pub time: f32,
    pub temperature_2m: f32,
    pub relativehumidity_2m: f32,
    pub apparent_temperature: f32,
    pub is_day: u8,
    pub precipitation: f32,
    pub rain: f32,
    pub showers: f32,
    pub snowfall: f32,
    pub weathercode: u8,
    pub cloudcover: f32,
    pub pressure_msl: f32,
    pub surface_pressure: f32,
    pub windspeed_10m: f32,
    pub winddirection_10m: f32,
    pub windgusts_10m: f32,
    pub uv_index: f32,
    pub uv_index_clear_sky: f32,
    pub cape: f32,
    pub freezinglevel_height: f32,
    pub shortwave_radiation: f32,
    pub direct_radiation: f32,
    pub diffuse_radiation: f32,
    pub direct_normal_irradiance: f32,
    pub terrestrial_radiation: f32,
    pub shortwave_radiation_instant: f32,
    pub direct_radiation_instant: f32,
    pub diffuse_radiation_instant: f32,
    pub direct_normal_irradiance_instant: f32,
    pub terrestrial_radiation_instant: f32,
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

pub async fn request_weather(latitude: String, longitude: String) -> Result<MeteoData, Box<dyn std::error::Error>> {
    let mut url = String::from("http://api.open-meteo.com/v1/forecast?latitude=");
    url.push_str(&latitude);
    url.push_str("&longitude=");
    url.push_str(&longitude);
    url.push_str("&timeformat=unixtime&models=best_match&current=temperature_2m,relativehumidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weathercode,cloudcover,pressure_msl,surface_pressure,windspeed_10m,winddirection_10m,windgusts_10m,uv_index,uv_index_clear_sky,cape,freezinglevel_height,shortwave_radiation,direct_radiation,diffuse_radiation,direct_normal_irradiance,terrestrial_radiation,shortwave_radiation_instant,direct_radiation_instant,diffuse_radiation_instant,direct_normal_irradiance_instant,terrestrial_radiation_instant");

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
    let result = reqwest::blocking::get(url)?;
    let parsed = result.json::<Vec<GeoData>>()?;

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

        assert_eq!(parsed_data.current.temperature_2m, 14.2);
        assert_eq!(parsed_data.current.windspeed_10m, 8.3);
    }

    // #[test]
    // fn hourly_weather() {
    //     let test_data = fs::read_to_string("test/weather_data.json").expect("Test file not found");
    //     let parsed_data: MeteoData = serde_json::from_str(&test_data).unwrap();

    //     assert_eq!(parsed_data.hourly.time[0], "2023-10-08T00:00");
    //     assert_eq!(parsed_data.hourly.temperature_2m[0], 11.6);
    //     assert_eq!(parsed_data.hourly.relativehumidity_2m[0], 83.0);
    // }

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
