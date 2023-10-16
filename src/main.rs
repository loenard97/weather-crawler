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

    // general data
    pub static ref LATITUDE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref LONGITUDE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref ELEVATION: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref GENERATION_TIME: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();

    // current weather data
    // pub static ref TIME: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref TEMPERATURE_2M: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref RELATIVEHUMIDITY_2M: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref APPARENT_TEMPERATURE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref IS_DAY: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref PRECIPITATION: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref RAIN: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref SHOWERS: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref SNOWFALL: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref WEATHERCODE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref CLOUDCOVER: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref PRESSURE_MSL: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref SURFACE_PRESSURE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref WINDSPEED_10M: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref WINDDIRECTION_10M: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref WINDGUSTS_10M: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref UV_INDEX: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref UV_INDEX_CLEAR_SKY: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref CAPE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref FREEZINGLEVEL_HEIGHT: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref SHORTWAVE_RADIATION: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref DIRECT_RADIATION: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref DIFFUSE_RADIATION: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref DIRECT_NORMAL_IRRADIANCE: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref TERRESTRIAL_RADIATION: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref SHORTWAVE_RADIATION_INSTANT: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref DIRECT_RADIATION_INSTANT: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref DIFFUSE_RADIATION_INSTANT: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref DIRECT_NORMAL_IRRADIANCE_INSTANT: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    pub static ref TERRESTRIAL_RADIATION_INSTANT: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    city: String,
}

fn register_metrics() {
    // general data
    REGISTRY.lock().unwrap().register("LATITUDE", "latitude", LATITUDE.clone());
    REGISTRY.lock().unwrap().register("LONGITUDE", "longitude", LONGITUDE.clone());
    REGISTRY.lock().unwrap().register("ELEVATION", "elevation", ELEVATION.clone());
    REGISTRY.lock().unwrap().register("GENERATION_TIME", "generation_time", GENERATION_TIME.clone());

    // current weather data
    // REGISTRY.lock().unwrap().register("_", "ö", TIME.clone());
    REGISTRY.lock().unwrap().register("TEMPERATURE_2M", "temperature_2m", TEMPERATURE_2M.clone());
    REGISTRY.lock().unwrap().register("RELATIVEHUMIDITY_2M", "relativehumidity_2m", RELATIVEHUMIDITY_2M.clone());
    REGISTRY.lock().unwrap().register("APPARENT_TEMPERATURE", "apparent_temperature", APPARENT_TEMPERATURE.clone());
    REGISTRY.lock().unwrap().register("IS_DAY", "is_day", IS_DAY.clone());
    REGISTRY.lock().unwrap().register("PRECIPITATION", "precipitation", PRECIPITATION.clone());
    REGISTRY.lock().unwrap().register("RAIN", "rain", RAIN.clone());
    REGISTRY.lock().unwrap().register("SHOWERS", "showers", SHOWERS.clone());
    REGISTRY.lock().unwrap().register("SNOWFALL", "snowfall", SNOWFALL.clone());
    REGISTRY.lock().unwrap().register("WEATHERCODE", "weathercode", WEATHERCODE.clone());
    REGISTRY.lock().unwrap().register("CLOUDCOVER", "cloudcover", CLOUDCOVER.clone());
    REGISTRY.lock().unwrap().register("PRESSURE_MSL", "pressure_msl", PRESSURE_MSL.clone());
    REGISTRY.lock().unwrap().register("SURFACE_PRESSURE", "surface_pressure", SURFACE_PRESSURE.clone());
    REGISTRY.lock().unwrap().register("WINDSPEED_10M", "windspeed_10m", WINDSPEED_10M.clone());
    REGISTRY.lock().unwrap().register("WINDDIRECTION_10M", "winddirection_10m", WINDDIRECTION_10M.clone());
    REGISTRY.lock().unwrap().register("WINDGUSTS_10M", "windgusts_10m", WINDGUSTS_10M.clone());
    REGISTRY.lock().unwrap().register("UV_INDEX", "uv_index", UV_INDEX.clone());
    REGISTRY.lock().unwrap().register("UV_INDEX_CLEAR_SKY", "uv_index_clear_sky", UV_INDEX_CLEAR_SKY.clone());
    REGISTRY.lock().unwrap().register("CAPE", "cape", CAPE.clone());
    REGISTRY.lock().unwrap().register("FREEZINGLEVEL_HEIGHT", "freezinglevel_height", FREEZINGLEVEL_HEIGHT.clone());
    REGISTRY.lock().unwrap().register("SHORTWAVE_RADIATION", "shortwave_radiation", SHORTWAVE_RADIATION.clone());
    REGISTRY.lock().unwrap().register("DIRECT_RADIATION", "direct_radiation", DIRECT_RADIATION.clone());
    REGISTRY.lock().unwrap().register("DIFFUSE_RADIATION", "diffuse_radiation", DIFFUSE_RADIATION.clone());
    REGISTRY.lock().unwrap().register("DIRECT_NORMAL_IRRADIANCE", "direct_normal_irradiance", DIRECT_NORMAL_IRRADIANCE.clone());
    REGISTRY.lock().unwrap().register("TERRESTRIAL_RADIATION", "terrestrial_radiation", TERRESTRIAL_RADIATION.clone());
    REGISTRY.lock().unwrap().register("SHORTWAVE_RADIATION_INSTANT", "shortwave_radiation_instant", SHORTWAVE_RADIATION_INSTANT.clone());
    REGISTRY.lock().unwrap().register("DIRECT_RADIATION_INSTANT", "direct_radiation_instant", DIRECT_RADIATION_INSTANT.clone());
    REGISTRY.lock().unwrap().register("DIFFUSE_RADIATION_INSTANT", "diffuse_radiation_instant", DIFFUSE_RADIATION_INSTANT.clone());
    REGISTRY.lock().unwrap().register("DIRECT_NORMAL_IRRADIANCE_INSTANT", "direct_normal_irradiance_instant", DIRECT_NORMAL_IRRADIANCE_INSTANT.clone());
    REGISTRY.lock().unwrap().register("TERRESTRIAL_RADIATION_INSTANT", "terrestrial_radiation_instant", TERRESTRIAL_RADIATION_INSTANT.clone());
}

#[get("/metrics")]
async fn prometheus() -> String {
    let latitude = &GEO_LOCATION.lock().unwrap().lat.clone();
    let longitude = &GEO_LOCATION.lock().unwrap().lon.clone();

    let weather_data = request_weather(latitude.to_string(), longitude.to_string())
        .await
        .unwrap();
    let current_weather = weather_data.current;

    // general data
    LATITUDE.set(weather_data.latitude.into());
    LONGITUDE.set(weather_data.longitude.into());
    ELEVATION.set(weather_data.elevation.into());
    GENERATION_TIME.set(weather_data.generationtime_ms.into());

    // current weather data
    // pub static ref TIME: Gauge::<f64, AtomicU64> = Gauge::<f64, AtomicU64>::default();
    TEMPERATURE_2M.set(current_weather.temperature_2m.into());
    RELATIVEHUMIDITY_2M.set(current_weather.relativehumidity_2m.into());
    APPARENT_TEMPERATURE.set(current_weather.apparent_temperature.into());
    IS_DAY.set(current_weather.is_day.into());
    PRECIPITATION.set(current_weather.precipitation.into());
    RAIN.set(current_weather.rain.into());
    SHOWERS.set(current_weather.showers.into());
    SNOWFALL.set(current_weather.snowfall.into());
    WEATHERCODE.set(current_weather.weathercode.into());
    CLOUDCOVER.set(current_weather.cloudcover.into());
    PRESSURE_MSL.set(current_weather.pressure_msl.into());
    SURFACE_PRESSURE.set(current_weather.surface_pressure.into());
    WINDSPEED_10M.set(current_weather.windspeed_10m.into());
    WINDDIRECTION_10M.set(current_weather.winddirection_10m.into());
    WINDGUSTS_10M.set(current_weather.windgusts_10m.into());
    UV_INDEX.set(current_weather.uv_index.into());
    UV_INDEX_CLEAR_SKY.set(current_weather.uv_index_clear_sky.into());
    CAPE.set(current_weather.cape.into());
    FREEZINGLEVEL_HEIGHT.set(current_weather.freezinglevel_height.into());
    SHORTWAVE_RADIATION.set(current_weather.shortwave_radiation.into());
    DIRECT_RADIATION.set(current_weather.direct_radiation.into());
    DIFFUSE_RADIATION.set(current_weather.diffuse_radiation.into());
    DIRECT_NORMAL_IRRADIANCE.set(current_weather.direct_normal_irradiance.into());
    TERRESTRIAL_RADIATION.set(current_weather.terrestrial_radiation.into());
    SHORTWAVE_RADIATION_INSTANT.set(current_weather.shortwave_radiation_instant.into());
    DIRECT_RADIATION_INSTANT.set(current_weather.direct_radiation_instant.into());
    DIFFUSE_RADIATION_INSTANT.set(current_weather.diffuse_radiation_instant.into());
    DIRECT_NORMAL_IRRADIANCE_INSTANT.set(current_weather.direct_normal_irradiance_instant.into());
    TERRESTRIAL_RADIATION_INSTANT.set(current_weather.terrestrial_radiation_instant.into());

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

    register_metrics();

    rocket::build().mount("/", routes![prometheus])
}
