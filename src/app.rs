// if cargo stuck on 'Blocking waiting for file lock on package cache, use
// rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.add_space(12.5);
                    ui.menu_button("Roster Management", |ui| {
                        if ui.button("Help").clicked() {}
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.add_space(13.0);
            ui.label("RST");
            ui.add_space(0.5);
            ui.label("Manager");
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);
            ui.group(|ui| {
                // ui.label
                if ui.button("👤 Profile").clicked() {
                    println!("Profile configuration presssed!");
                }
                ui.add_space(2.0);

                if ui.button("ℹ School Info").clicked() {
                    println!("School info clicked!");
                    // TODO: Implement school info
                }

                if ui.button("📒 Ledger").clicked() {
                    println!("Ledger!");
                    // TODO: Implement Ledger
                }

                ui.add_space(2.0);
                if ui.button("📢 Report").clicked() {} // maybe Discipline
                ui.add_space(2.0);
                if ui.button("✉ Message").clicked() {} // maybe Comments
                ui.add_space(2.0);
                if ui.button("✆ Directory").clicked() {} // maybe Directory
                ui.add_space(2.0);
                if ui.button("♖ Events").clicked() {} // maybe Events
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.separator();
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.label("test");
        });
    }
}
