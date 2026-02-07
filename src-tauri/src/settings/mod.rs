use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf, sync::{Mutex, MutexGuard}};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub theme: Theme,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { theme: Theme::Dark }
    }
}

#[derive(Debug)]
pub enum SettingsError {
    ConfigDirUnavailable,
    Io(io::Error),
    Json(serde_json::Error),
    LockPoisoned,
    Validation(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::ConfigDirUnavailable => {
                write!(f, "platform config directory unavailable")
            }
            SettingsError::Io(e) => write!(f, "io error: {e}"),
            SettingsError::Json(e) => write!(f, "json error: {e}"),
            SettingsError::LockPoisoned => write!(f, "settings store lock poisoned"),
            SettingsError::Validation(msg) => write!(f, "validation error: {msg}"),
        }
    }
}

impl std::error::Error for SettingsError {}

impl From<io::Error> for SettingsError {
    fn from(value: io::Error) -> Self {
        SettingsError::Io(value)
    }
}

impl From<serde_json::Error> for SettingsError {
    fn from(value: serde_json::Error) -> Self {
        SettingsError::Json(value)
    }
}

#[derive(Debug)]
pub struct SettingsStore {
    base_dir: Option<PathBuf>,
    lock: Mutex<()>,
}

impl SettingsStore {
    pub fn new() -> Self {
        let base_dir = dirs::config_dir().map(|p| p.join("multivm-workspace"));
        Self {
            base_dir,
            lock: Mutex::new(()),
        }
    }

    pub fn get(&self) -> AppSettings {
        match self.get_result() {
            Ok(s) => s,
            Err(_) => AppSettings::default(),
        }
    }

    pub fn set_theme_result(&self, theme: Theme) -> Result<AppSettings, SettingsError> {
        let _guard = self.lock_guard()?;
        let settings = AppSettings { theme };
        self.write_settings(&settings)?;
        Ok(settings)
    }

    pub fn parse_theme(theme: &str) -> Result<Theme, SettingsError> {
        match theme.trim().to_lowercase().as_str() {
            "dark" => Ok(Theme::Dark),
            "light" => Ok(Theme::Light),
            other => Err(SettingsError::Validation(format!(
                "invalid theme: {other}"
            ))),
        }
    }

    fn get_result(&self) -> Result<AppSettings, SettingsError> {
        let _guard = self.lock_guard()?;
        let path = self.settings_path()?;
        match fs::read_to_string(&path) {
            Ok(s) => {
                let parsed: AppSettings = serde_json::from_str(&s)?;
                Ok(parsed)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(AppSettings::default()),
            Err(e) => Err(SettingsError::Io(e)),
        }
    }

    fn lock_guard(&self) -> Result<MutexGuard<'_, ()>, SettingsError> {
        self.lock.lock().map_err(|_| SettingsError::LockPoisoned)
    }

    fn ensure_dir(&self) -> Result<PathBuf, SettingsError> {
        let base = self
            .base_dir
            .clone()
            .ok_or(SettingsError::ConfigDirUnavailable)?;
        fs::create_dir_all(&base)?;
        Ok(base)
    }

    fn settings_path(&self) -> Result<PathBuf, SettingsError> {
        let base = self.ensure_dir()?;
        Ok(base.join("settings.json"))
    }

    fn write_settings(&self, settings: &AppSettings) -> Result<(), SettingsError> {
        let path = self.settings_path()?;
        let tmp = path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(&tmp, json.as_bytes())?;

        if fs::rename(&tmp, &path).is_err() {
            // If rename fails (commonly on Windows if dest exists), fall back to overwrite.
            let bytes = fs::read(&tmp)?;
            fs::write(&path, bytes)?;
            let _ = fs::remove_file(&tmp);
        }
        Ok(())
    }
}
