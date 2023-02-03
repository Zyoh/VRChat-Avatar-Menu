// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use crate::params::{self, SavedParameter};

use vrchat_osc as osc;

use std::{error::Error, path::{PathBuf, Path}};
use eframe::epaint::ahash::HashMap;

const DEFAULT_TO_VRCHAT_IP: &str = "127.0.0.1:9000";

pub fn launch() {
    let eoptions = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(600.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "VRChat Avatar",
        eoptions,
        Box::new(|_cc| Box::<App>::default()),
    );
}

#[derive(Default)]
struct App {
    avatar_osc_config_path: Option<PathBuf>,
    params: HashMap<String, osc::Value>,
    filter_name: String,
    target_ip: String,
    engine: Option<osc::Engine>,
}

impl App {
    fn avatar_id(&self) -> Option<String> {
        if let Some(path) = &self.avatar_osc_config_path {
            if let Some(id) = path.file_stem() {
                return Some(id.to_string_lossy().to_string());
            }
        }

        None
    }

    fn avatar_data_path(&self) -> Option<PathBuf> {
        // Path to avatar 3.0 save data for the current avatar (based on loaded config)

        if let Some(config_path) = self.avatar_osc_config_path.as_deref() {
            if let (
                Some(Some(user_id)),
                Some(vrchat_folder),
                Some(id)
            )
            = (
                config_path.ancestors().nth(2).map(|v| v.file_stem()), 
                config_path.ancestors().nth(4), 
                self.avatar_id()
            ) {
                let user_id = user_id.to_string_lossy();
                let avatar_data_file = vrchat_folder
                    .join(Path::new("LocalAvatarData"))
                    .join(Path::new(user_id.as_ref()))
                    .join(Path::new(&id));

                return Some(avatar_data_file);
            }
        }

        None
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VRChat Avatar");

            // Connection
            ui.horizontal(|ui| {
                ui.monospace("Target IP: ");
                if let Some(engine) = &self.engine {
                    ui.monospace(engine.vrchat_listens_to_addr.clone());
                } else {
                    ui.monospace("None");
                }
            });

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.target_ip);

                if ui.button("Connect").clicked() {
                    if self.target_ip.is_empty() {
                        self.target_ip = DEFAULT_TO_VRCHAT_IP.to_string();
                    }
                    ui.monospace("Connecting...");
                    self.engine = Some(osc::Engine {
                        vrchat_listens_to_addr: self.target_ip.clone(),
                        ..Default::default()
                    });
                }
            });
            

            if self.engine.is_none(){ 
                return;
            }

            // Avatar config

            // TODO: Open to VRChat OSC folder to make finding the config easier
            if ui.button("Choose avatar config...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.avatar_osc_config_path = Some(path.clone());
                    self.params.clear();

                    for param in params::get_avatar_params(&path).unwrap() {
                        match param.ptype.to_lowercase().as_str() {
                            "float" => {
                                self.params.insert(param.name, osc::Value::Float(0.0));
                            }
                            "int" => {
                                self.params.insert(param.name, osc::Value::Int(0));
                            }
                            "bool" => {
                                self.params.insert(param.name, osc::Value::Bool(false));
                            }
                            _ => {}
                        }
                    }

                    // Load saved params
                    if let Some(avatar_data_path) = self.avatar_data_path() {
                    if let Ok(saved_params) = SavedParameter::from_file(&avatar_data_path) {
                        for saved_param in saved_params {
                            if let Some(value) = self.params.get_mut(saved_param.name.as_str()) {
                                match value {
                                    osc::Value::Float(v) => {
                                        *v = saved_param.raw_value;
                                    }
                                    osc::Value::Int(v) => {
                                        *v = saved_param.raw_value as u8;
                                    }
                                    osc::Value::Bool(v) => {
                                        *v = saved_param.raw_value >= 0.5;
                                    }
                                }
                            }
                        }
                    }
                    }
                }
            }

            if let Some(id) = self.avatar_id() {
                ui.monospace(format!("Avatar ID: {id}"));
            } else {
                return;
            }

            ui.monospace(format!("Found {:?} params", self.params.len()));

            ui.horizontal(|ui| {
                ui.monospace("Filter: ");
                ui.text_edit_singleline(&mut self.filter_name);
            });

            let engine = self.engine.as_ref().unwrap();

            // Scrollable area
            egui::ScrollArea::vertical().auto_shrink([true, true]).show(ui, |ui| {
                ui.vertical(|ui| {
                    let mut params: Vec<(&String, &mut osc::Value)> = self.params.iter_mut().collect();
                    params.sort_by(|a, b| a.0.cmp(b.0));

                    for (name, value) in params {
                        if !name.starts_with(self.filter_name.as_str()) {
                            continue;
                        }
                        
                        ui.horizontal(|ui| {
                            ui.monospace(name);

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                match value {
                                    osc::Value::Float(mut v) => {
                                        let slider = egui::Slider::new(&mut v, 0.0..=1.0);
                                        ui.add(slider);
            
                                        if value != &osc::Value::Float(v) {
                                            *value = osc::Value::Float(v);
                                            on_change(engine, name.clone(), value.clone()).unwrap();
                                        }
                                    },
                                    osc::Value::Bool(mut v) => {
                                        ui.checkbox(&mut v, "");
                                        
                                        if value != &osc::Value::Bool(v) {
                                            *value = osc::Value::Bool(v);
                                            on_change(engine, name.clone(), value.clone()).unwrap();
                                        }
                                    },
                                    osc::Value::Int(mut v) => {
                                        ui.add(egui::Slider::new(&mut v, 0..=255));
                                        
                                        if value != &osc::Value::Int(v) {
                                            *value = osc::Value::Int(v);
                                            on_change(engine, name.clone(), value.clone()).unwrap();
                                        }
                                    },
                                }
                            });
                        });
                    }
                });
            });
        });
    }
}

fn on_change(engine: &osc::Engine, addr: String, value: osc::Value) -> Result<(), Box<dyn Error>> {
    engine.send(osc::Input::Avatar(addr, value))?;

    Ok(())
}
