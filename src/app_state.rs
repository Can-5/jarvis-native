use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    Chat,
    Voice,
    System,
    Settings,
    Camera,
    Security,
    #[serde(rename = "data_studio")]
    DataStudio,
    #[serde(rename = "system_monitor")]
    SystemMonitor,
    #[serde(rename = "camera_analysis")]
    CameraAnalysis,
    #[serde(rename = "voice_analysis")]
    VoiceAnalysis,
    #[serde(rename = "data_analysis")]
    DataAnalysis,
    Presenter,
    Designer,
    Child,
    Adult,
    Producer,
    Consumer,
    Artist,
    Musician,
    Doctor,
    Lawyer,
    Programmer,
    Traveler,
    Philosopher,
    #[serde(rename = "child_beginner")]
    ChildBeginner,
    Player,
    Prosecutor,
    #[serde(rename = "non_programmer")]
    NonProgrammer,
    Politician,
    Wealthy,
    Poor,
    #[serde(rename = "rapper_content_creator")]
    RapperContentCreator,
    #[serde(rename = "site_creator")]
    SiteCreator,
    #[serde(rename = "normal_person")]
    NormalPerson,
    Elderly,
    Young,
    #[serde(rename = "google_data_collector")]
    GoogleDataCollector,
    #[serde(rename = "linux_unknown")]
    LinuxUnknown,
    Bluetooth,
    Wifi,
    Web,
    VoiceCommand,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Chat
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Chat => write!(f, "SOHBET"),
            Mode::Voice => write!(f, "SES"),
            Mode::System => write!(f, "SİSTEM"),
            Mode::Settings => write!(f, "AYARLAR"),
            Mode::Camera => write!(f, "KAMERA"),
            Mode::Security => write!(f, "GÜVENLİK"),
            Mode::DataStudio => write!(f, "VERİ STÜDYO"),
            Mode::SystemMonitor => write!(f, "İZ SİSTEM"),
            Mode::CameraAnalysis => write!(f, "KAMERA ANALİZ"),
            Mode::VoiceAnalysis => write!(f, "SES ANALİZ"),
            Mode::DataAnalysis => write!(f, "VERİ ANALİZ"),
            Mode::Presenter => write!(f, "SUNUMCU"),
            Mode::Designer => write!(f, "TASARIMCI"),
            Mode::Child => write!(f, "ÇOCUK"),
            Mode::Adult => write!(f, "YETİŞKİN"),
            Mode::Producer => write!(f, "ÜRETİCİ"),
            Mode::Consumer => write!(f, "TÜKETİCİ"),
            Mode::Artist => write!(f, "SANATCI"),
            Mode::Musician => write!(f, "MÜZİKÇİ"),
            Mode::Doctor => write!(f, "DOKTOR"),
            Mode::Lawyer => write!(f, "AVUKAT"),
            Mode::Programmer => write!(f, "PROGRAMCI"),
            Mode::Traveler => write!(f, "SEYAHATÇİ"),
            Mode::Philosopher => write!(f, "FİLOZOF"),
            Mode::ChildBeginner => write!(f, "ÇOCUK (AZ BİLGEN)"),
            Mode::Player => write!(f, "OYUNCU"),
            Mode::Prosecutor => write!(f, "SAVCI"),
            Mode::NonProgrammer => write!(f, "KOD BİLMEYEN"),
            Mode::Politician => write!(f, "SİYASETCİ"),
            Mode::Wealthy => write!(f, "ZENGİN"),
            Mode::Poor => write!(f, "FAKİR"),
            Mode::RapperContentCreator => write!(f, "RAPÇI İÇERİK ÜRETİCİ"),
            Mode::SiteCreator => write!(f, "SITE ÜRETİCİ"),
            Mode::NormalPerson => write!(f, "NORMAL ADAM"),
            Mode::Elderly => write!(f, "YAŞLI"),
            Mode::Young => write!(f, "GENÇ"),
            Mode::GoogleDataCollector => write!(f, "GOOGLE VERİ TOPLAYAN"),
            Mode::LinuxUnknown => write!(f, "LINUX BİLMEYEN"),
            Mode::Bluetooth => write!(f, "BLUETOOTH"),
            Mode::Wifi => write!(f, "WIFI"),
            Mode::Web => write!(f, "WEB"),
            Mode::VoiceCommand => write!(f, "SES KOMUT"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub from: String,
    pub text: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemStats {
    pub cpu: f32,
    pub mem: f32,
    pub disk: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Personality {
    pub personality: String,
    pub mood: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub voice: String,
    pub hotkey: String,
    pub developer_mode: bool,
    pub auto_error_report: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub mode: Mode,
    pub chat_history: Vec<crate::ChatMessage>,
    pub input: String,
    pub system_stats: SystemStats,
    pub personality: crate::Personality,
    pub settings: crate::AppSettings,
    pub komut: String,
    pub usb_devices: String,
    pub repair_result: String,
    pub scan_result: String,
    pub usb_src: String,
    pub usb_dst: String,
    pub fs_device: String,
    pub fs_scan_path: String,
    pub guvenlik_sonuc: String,
    pub protection_acik: bool,
    pub camera_width: u32,
    pub camera_height: u32,
    pub snapshot_interval: u32,
    pub snapshot_count: u32,
    pub qr_result: Option<String>,
    pub vision_result: Option<String>,
    pub vision_loading: bool,
    pub camera_active: bool,
    pub photos: Vec<Photo>,
    pub stats: SystemStats,
    pub qr_result_str: Option<String>,
    pub vision_result_str: Option<String>,
    pub vision_loading_bool: bool,
    pub camera_active_bool: bool,
    pub photos_vec: Vec<Photo>,
    pub stats_sys: SystemStats,
    pub voice_mode_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Photo {
    pub data_url: String,
    pub zaman: String,
}

impl AppState {
    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn chat_history(&self) -> &Vec<crate::ChatMessage> {
        &self.chat_history
    }

    pub fn chat_history_mut(&mut self) -> &mut Vec<crate::ChatMessage> {
        &mut self.chat_history
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut String {
        &mut self.input
    }

    pub fn system_stats(&self) -> &SystemStats {
        &self.system_stats
    }

    pub fn system_stats_mut(&mut self) -> &mut SystemStats {
        &mut self.system_stats
    }

    pub fn personality(&self) -> &crate::Personality {
        &self.personality
    }

    pub fn personality_mut(&mut self) -> &mut crate::Personality {
        &mut self.personality
    }

    pub fn settings(&self) -> &crate::AppSettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut crate::AppSettings {
        &mut self.settings
    }

    pub fn komut(&self) -> &str {
        &self.komut
    }

    pub fn komut_mut(&mut self) -> &mut String {
        &mut self.komut
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Photo {
    pub data_url: String,
    pub zaman: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatMessage {
    pub from: String,
    pub text: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemStats {
    pub cpu: f32,
    pub mem: f32,
    pub disk: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Personality {
    pub personality: String,
    pub mood: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub voice: String,
    pub hotkey: String,
    pub developer_mode: bool,
    pub auto_error_report: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Photo {
    pub data_url: String,
    pub zaman: String,
}