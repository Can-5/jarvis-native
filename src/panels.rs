use crate::app_state::{AppState, Mode, ChatMessage, SystemStats};
use egui::{Color32, RichText, ScrollArea, Ui, vec2};

pub fn left_panel(ui: &mut eframe::egui::Ui, state: &mut AppState) {
    ui.heading(egui::RichText::new("J.A.R.V.I.S.").size(24.0).color(Color32::from_rgb(0, 212, 255)).strong());
    ui.separator();

    let modes = [
        Mode::Chat, Mode::Voice, Mode::System, Mode::Settings, Mode::Camera, Mode::Security,
        Mode::DataStudio, Mode::SystemMonitor, Mode::CameraAnalysis, Mode::VoiceAnalysis,
        Mode::DataAnalysis, Mode::Presenter, Mode::Designer, Mode::Child, Mode::Adult,
        Mode::Producer, Mode::Consumer, Mode::Artist, Mode::Musician, Mode::Doctor,
        Mode::Lawyer, Mode::Programmer, Mode::Traveler, Mode::Philosopher,
        Mode::ChildBeginner, Mode::Player, Mode::Prosecutor, Mode::NonProgrammer,
        Mode::Politician, Mode::Wealthy, Mode::Poor, Mode::RapperContentCreator,
        Mode::SiteCreator, Mode::NormalPerson, Mode::Elderly, Mode::Young,
        Mode::GoogleDataCollector, Mode::LinuxUnknown, Mode::Bluetooth, Mode::Wifi,
        Mode::Web, Mode::System, Mode::VoiceCommand, Mode::Settings,
    ];

    for mode in modes {
        let selected = state.mode == mode;
        if ui.selectable_label(selected, mode.to_string()).clicked() {
            state.mode = mode;
        }
    }

    ui.separator();

    let stats = state.system_stats();
    ui.label(format!("CPU: {:.0}%", stats.cpu));
    ui.label(format!("RAM: {:.0}%", stats.mem));
    ui.label(format!("Disk: {:.0}%", stats.disk));
}

pub fn central_panel(ui: &mut eframe::egui::Ui, state: &mut crate::app_state::AppState) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        ui.label(egui::RichText::new("J.A.R.V.I.S.").size(48.0).color(egui::Color32::from_rgb(0, 212, 255)));
        ui.add_space(20.0);
        ui.label("Hazır. Komut bekliyorum...");
    });

    // Chat history area
    egui::ScrollArea::vertical().show(ui, |ui| {
        for msg in state.chat_history() {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&msg.from).strong());
                ui.label(&msg.text);
            });
            ui.add_space(4.0);
        }
    });
}

pub fn right_panel(ui: &mut eframe::egui::Ui, state: &mut crate::app_state::AppState) {
    match state.mode {
        crate::app_state::Mode::Settings => {
            ui.heading("⚙️ Ayarlar");
            ui.separator();

            // Theme
            ui.label("Tema");
            egui::ComboBox::from_label("")
                .selected_text(&state.settings().theme)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.settings_mut().theme, "dark".to_string(), "Koyu");
                    ui.selectable_value(&mut state.settings_mut().theme, "light".to_string(), "Açık");
                    ui.selectable_value(&mut state.settings_mut().theme, "system".to_string(), "Sistem");
                });

            ui.add_space(10.0);
            // Language
            ui.label("Dil");
            egui::ComboBox::from_label("")
                .selected_text(&state.settings().language)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.settings_mut().language, "tr".to_string(), "Türkçe");
                    ui.selectable_value(&mut state.settings_mut().language, "en".to_string(), "English");
                });

            ui.add_space(10.0);
            // Voice
            ui.label("Ses (TTS)");
            egui::ComboBox::from_label("")
                .selected_text(&state.settings().voice)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.settings_mut().voice, "default".to_string(), "Varsayılan");
                    ui.selectable_value(&mut state.settings_mut().voice, "en-US-ryan".to_string(), "Ryan (EN)");
                    ui.selectable_value(&mut state.settings_mut().voice, "tr-TR-emre".to_string(), "Emre (TR)");
                });

            ui.add_space(10.0);
            // Hotkey
            ui.label("Kısayol (Hot-key)");
            ui.text_edit_singleline(&mut state.settings_mut().hotkey);

            ui.add_space(20.0);
            // Personality
            ui.label("Kişilik");
            egui::ComboBox::from_label("")
                .selected_text(&state.personality().personality)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.personality_mut().personality, "friendly".to_string(), "Samimi");
                    ui.selectable_value(&mut state.personality_mut().personality, "professional".to_string(), "Profesyonel");
                    ui.selectable_value(&mut state.personality_mut().personality, "humorous".to_string(), "Mizahi");
                    ui.selectable_value(&mut state.personality_mut().personality, "strict".to_string(), "Katı");
                    ui.selectable_value(&mut state.personality_mut().personality, "empathetic".to_string(), "Empati");
                    ui.selectable_value(&mut state.personality_mut().personality, "curious".to_string(), "Meraklı");
                    ui.selectable_value(&mut state.personality_mut().personality, "confident".to_string(), "Kendine Güvenen");
                });

            ui.add_space(5.0);
            ui.label("Ruh Hali");
            egui::ComboBox::from_label("")
                .selected_text(&state.personality().mood)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.personality_mut().mood, "notr".to_string(), "Nötr");
                    ui.selectable_value(&mut state.personality_mut().mood, "mutlu".to_string(), "Mutlu");
                    ui.selectable_value(&mut state.personality_mut().mood, "uzgun".to_string(), "Üzgün");
                    ui.selectable_value(&mut state.personality_mut().mood, "heyecanli".to_string(), "Heyecanlı");
                    ui.selectable_value(&mut state.personality_mut().mood, "sakin".to_string(), "Sakin");
                    ui.selectable_value(&mut state.personality_mut().mood, "enerjik".to_string(), "Enerjik");
                    ui.selectable_value(&mut state.personality_mut().mood, "merakli".to_string(), "Meraklı");
                });
        }
        _ => {
            ui.heading("Mod Bilgisi");
            ui.label(format!("Aktif mod: {}", state.mode.to_string()));
            ui.add_space(10.0);
            ui.label("Bu mod için özel ayarlar buraya eklenecek.");
        }
    }
}

pub fn bottom_panel(ui: &mut eframe::egui::Ui, state: &mut crate::app_state::AppState) {
    ui.horizontal(|ui| {
        let response = ui.add(
            egui::TextEdit::singleline(state.input_mut())
                .hint_text("Komut yazın... (ör: 'kamera aç', 'koruma aç')")
                .desired_width(f32::INFINITY),
        );
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            handle_command(state, state.input.clone());
            state.input.clear();
        }
    });
}

fn handle_command(state: &mut crate::app_state::AppState, cmd: String) {
    state.chat_history_mut().push(crate::app_state::ChatMessage {
        from: "Siz".to_string(),
        text: cmd.clone(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    });

    let reply = match cmd.to_lowercase().as_str() {
        c if c.contains("merhaba") => "Merhaba! Nasıl yardımcı olabilirim?".to_string(),
        c if c.contains("nasılsın") => "İyiyim, teşekkürler! Sen nasılsın?".to_string(),
        _ => format!("Komut alındı: '{}'. Henüz işlenmedi.", cmd),
    };

    state.chat_history_mut().push(crate::app_state::ChatMessage {
        from: "JARVIS".to_string(),
        text: reply,
        timestamp: chrono::Utc::now().timestamp() as u64,
    });
}