use serde::Deserialize;

const TOKYO_WEATHER_URL: &str = "https://weather.tsukumijima.net/api/forecast/city/130010";
const HTTP_FLAG: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root: Root = if HTTP_FLAG {
        reqwest::blocking::get(TOKYO_WEATHER_URL)?.json()?
    } else {
        let rf = std::fs::read_to_string("w.json")?;
        serde_json::from_str(&rf)?
    };

    println!("{}:", root.title);

    for f in &root.forecasts {
        let min = fmt_temp(&f.temperature.min.celsius);
        let max = fmt_temp(&f.temperature.max.celsius);

        println!(
            " {}: {}  最高 {}℃ / 最低 {}℃",
            f.date,
            convert2emoji(&f.telop),
            max,
            min
        );
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Root {
    title: String,
    forecasts: Vec<Forecast>,
}

#[derive(Debug, Deserialize)]
struct Forecast {
    date: String,
    telop: String,
    temperature: Temperature,
}

#[derive(Debug, Deserialize)]
struct Temperature {
    min: TempValue,
    max: TempValue,
}

#[derive(Debug, Deserialize)]
struct TempValue {
    celsius: Option<String>,
}

fn fmt_temp(t: &Option<String>) -> &str {
    t.as_deref().unwrap_or("--")
}

fn convert2emoji(weather_telop: &str) -> String {
    const DICT1: [(&str, &str); 7] = [
        ("曇り", "☁️"),
        ("晴れ", "☀️"),
        ("のち時々", "→"),
        ("のち一時", "→"),
        ("風雪", "☃️🌪"),
        ("暴風雪", "☃️🌪"),
        ("強い", ""),
    ];

    const DICT2: [(&str, &str); 10] = [
        ("か", "/"),
        ("時々", ""),
        ("一時", ""),
        ("のち", "→"),
        ("曇", "☁️"),
        ("止む", "☁️"),
        ("雨", "☂️"),
        ("晴", "☀️"),
        ("雪", "⛄️"),
        ("雷", "⚡️"),
    ];

    let result = DICT1
        .iter()
        .fold(weather_telop.to_string(), |s, (k, v)| s.replace(k, v));

    DICT2.iter().fold(result, |s, (k, v)| s.replace(k, v))
}
