#[macro_use]
extern crate rocket;

use clap::Parser;
use lazy_static::lazy_static;
use prometheus_client::encoding::text;
use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;

use weather_crawler::{request_geolocation, request_weather, GeoData};

lazy_static! {
    pub static ref GEO_LOCATION: Mutex<GeoData> = Mutex::new(GeoData {
        lat: "".to_string(),
        lon: "".to_string()
    });
    pub static ref REGISTRY: Mutex<Registry> = Mutex::new(Registry::default());
    pub static ref TEMPERATURE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref WINDSPEED: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    city: String,
}

fn register_metrics() {
    REGISTRY.lock().unwrap().register(
        "temperature",
        "Current temperature (°C)",
        TEMPERATURE.clone(),
    );
    REGISTRY
        .lock()
        .unwrap()
        .register("windspeed", "Current windspeed (m/s)", WINDSPEED.clone());
}

#[get("/")]
fn index() -> String {
    let mut url = String::from("https://api.open-meteo.com/v1/forecast?latitude=");
    url.push_str(&GEO_LOCATION.lock().unwrap().lat);
    url.push_str("&longitude=");
    url.push_str(&GEO_LOCATION.lock().unwrap().lon);
    url.push_str("&current_weather=true&hourly=temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,precipitation,rain,showers,snowfall,snow_depth,weathercode,pressure_msl,surface_pressure,cloudcover,cloudcover_low,cloudcover_mid,cloudcover_high,visibility,evapotranspiration,et0_fao_evapotranspiration,vapor_pressure_deficit,windspeed_10m,windspeed_80m,windspeed_120m,windspeed_180m,winddirection_10m,winddirection_80m,winddirection_120m,winddirection_180m,windgusts_10m,temperature_80m,temperature_120m,temperature_180m,soil_temperature_0cm,soil_temperature_6cm,soil_temperature_18cm,soil_temperature_54cm,soil_moisture_0_1cm,soil_moisture_1_3cm,soil_moisture_3_9cm,soil_moisture_9_27cm,soil_moisture_27_81cm,uv_index,uv_index_clear_sky,is_day,cape,freezinglevel_height,shortwave_radiation,direct_radiation,diffuse_radiation,direct_normal_irradiance,terrestrial_radiation&daily=weathercode,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,uv_index_max,uv_index_clear_sky_max,precipitation_sum,rain_sum,showers_sum,snowfall_sum,precipitation_hours,precipitation_probability_max,windspeed_10m_max,windgusts_10m_max,winddirection_10m_dominant,shortwave_radiation_sum,et0_fao_evapotranspiration&timezone=Europe%2FBerlin");

    url
}

#[get("/metrics")]
async fn prometheus() -> String {
    let mut url = String::from("https://api.open-meteo.com/v1/forecast?latitude=");
    url.push_str(&GEO_LOCATION.lock().unwrap().lat);
    url.push_str("&longitude=");
    url.push_str(&GEO_LOCATION.lock().unwrap().lon);
    url.push_str("&current_weather=true&hourly=temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,precipitation,rain,showers,snowfall,snow_depth,weathercode,pressure_msl,surface_pressure,cloudcover,cloudcover_low,cloudcover_mid,cloudcover_high,visibility,evapotranspiration,et0_fao_evapotranspiration,vapor_pressure_deficit,windspeed_10m,windspeed_80m,windspeed_120m,windspeed_180m,winddirection_10m,winddirection_80m,winddirection_120m,winddirection_180m,windgusts_10m,temperature_80m,temperature_120m,temperature_180m,soil_temperature_0cm,soil_temperature_6cm,soil_temperature_18cm,soil_temperature_54cm,soil_moisture_0_1cm,soil_moisture_1_3cm,soil_moisture_3_9cm,soil_moisture_9_27cm,soil_moisture_27_81cm,uv_index,uv_index_clear_sky,is_day,cape,freezinglevel_height,shortwave_radiation,direct_radiation,diffuse_radiation,direct_normal_irradiance,terrestrial_radiation&daily=weathercode,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,uv_index_max,uv_index_clear_sky_max,precipitation_sum,rain_sum,showers_sum,snowfall_sum,precipitation_hours,precipitation_probability_max,windspeed_10m_max,windgusts_10m_max,winddirection_10m_dominant,shortwave_radiation_sum,et0_fao_evapotranspiration&timezone=Europe%2FBerlin");

    let weather_data = request_weather(&url).await;
    let current_weather = weather_data.unwrap().current_weather;

    TEMPERATURE.set(current_weather.temperature.into());
    WINDSPEED.set(current_weather.windspeed.into());

    let mut response = String::new();
    text::encode(&mut response, &REGISTRY.lock().unwrap()).unwrap();

    response
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    let geo_location = &request_geolocation(&args.city).unwrap()[0];
    GEO_LOCATION
        .lock()
        .unwrap()
        .update(geo_location.lon.clone(), geo_location.lat.clone());

    println!("Found geo location: {:?}", GEO_LOCATION.lock().unwrap());

    register_metrics();

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![prometheus])
}
