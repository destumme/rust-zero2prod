use config::Environment;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub db: DatabaseSettings,
    pub app_port: u16
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new("config.yaml", config::FileFormat::Yaml)
        )
        .add_source(Environment::with_prefix("app_config"))
        .build()?;

    settings.try_deserialize::<Settings>()
}