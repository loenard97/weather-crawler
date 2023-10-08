<div align="center">

# weather-crawler
Prometheus client for weather data

![](https://img.shields.io/github/last-commit/loenard97/weather-crawler?&style=for-the-badge&logo=github&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/weather-crawler?&style=for-the-badge&logo=github&color=F74C00)

</div>


## Usage
Needs a city name or similar as command line arguement.
It then uses [geocode](https://geocode.maps.co/) to find latitude and longitude of that city on startup.
When done, a rocket http-server serves the address localhost:8000/prometheus_client, which scrapes and returns weather data from [Open Meteo](https://open-meteo.com/).

