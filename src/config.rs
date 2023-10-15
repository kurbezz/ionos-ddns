use once_cell::sync::Lazy;


fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}


pub struct Config {
    // https://developer.hosting.ionos.com/?source=IonosControlPanel
    pub ionos_public_prefix: String,
    pub ionos_secret: String,

    pub zone_name: String,
    pub record_name: Vec<String>,

    pub check_interval_seconds: u64,
}


impl Config {
    pub fn load() -> Config {
        Config {
            ionos_public_prefix: get_env("IONOS_PUBLIC_PREFIX"),
            ionos_secret: get_env("IONOS_SECRET"),

            zone_name: get_env("ZONE_NAME"),
            record_name: get_env("RECORD_NAME").split(",").map(|x| x.to_string()).collect(),

            check_interval_seconds: get_env("CHECK_INTERVAL_SECONDS").parse().unwrap(),
        }
    }
}


pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);
