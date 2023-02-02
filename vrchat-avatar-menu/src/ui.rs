// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use crate::params;

use vrchat_osc::{VRChatOSCType, VRChatOSC};

use std::{error::Error};
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
    json_path: Option<String>,
    param_values: HashMap<String, vrchat_osc::VRChatOSCType>,
    filter_name: String,
    target_ip: String,
    engine: Option<VRChatOSC>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VRChat Avatar");

            if self.engine.is_none() {
                ui.horizontal(|ui| {
                    ui.monospace("Target IP: ");
                    ui.text_edit_singleline(&mut self.target_ip);
                });                
            } else {
                ui.horizontal(|ui| {
                    ui.monospace("Target IP: ");
                    ui.monospace(&self.target_ip);
                });   
            }
            
            if self.engine.is_none() && ui.button("Connect").clicked() {
                if self.target_ip.is_empty() {
                    self.target_ip = DEFAULT_TO_VRCHAT_IP.to_string();
                }
                ui.monospace("Connecting...");
                self.engine = Some(VRChatOSC {
                    vrchat_listens_to_addr: self.target_ip.clone(),
                    ..Default::default()
                });
            }

            if self.engine.is_none(){ 
                return;
            }

            ui.spacing();

            // TODO: Open to VRChat OSC folder to make finding the config easier
            if ui.button("Choose avatar config...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.json_path = Some(path.display().to_string());
                    self.param_values.clear();

                    for param in params::get_avatar_params(&path).unwrap() {
                        match param.ptype.to_lowercase().as_str() {
                            "float" => {
                                self.param_values.insert(param.name, vrchat_osc::VRChatOSCType::Float(0.0));
                            }
                            "int" => {
                                self.param_values.insert(param.name, vrchat_osc::VRChatOSCType::Int(0));
                            }
                            "bool" => {
                                self.param_values.insert(param.name, vrchat_osc::VRChatOSCType::Bool(false));
                            }
                            _ => {}
                        }
                    }
                }
            }

            ui.monospace(format!("Found {:?} params", self.param_values.len()));

            ui.horizontal(|ui| {
                ui.monospace("Filter: ");
                ui.text_edit_singleline(&mut self.filter_name);
            });

            ui.spacing();

            let engine = self.engine.as_ref().unwrap();

            // Scrollable area
            egui::ScrollArea::vertical().auto_shrink([true, true]).show(ui, |ui| {
                ui.vertical(|ui| {
                    for (name, value) in self.param_values.iter_mut() {
                        if !name.starts_with(self.filter_name.as_str()) {
                            continue;
                        }
                        
                        ui.horizontal(|ui| {
                            match value {
                                VRChatOSCType::Float(mut v) => {
                                    let slider = egui::Slider::new(&mut v, 0.0..=1.0).text(name);
                                    ui.add(slider);
        
                                    if value != &VRChatOSCType::Float(v) {
                                        *value = VRChatOSCType::Float(v);
                                        on_change(engine, name.clone(), value.clone()).unwrap();
                                    }
                                },
                                VRChatOSCType::Bool(mut v) => {
                                    ui.checkbox(&mut v, name);
                                    
                                    if value != &VRChatOSCType::Bool(v) {
                                        *value = VRChatOSCType::Bool(v);
                                        on_change(engine, name.clone(), value.clone()).unwrap();
                                    }
                                },
                                VRChatOSCType::Int(mut v) => {
                                    ui.add(egui::Slider::new(&mut v, 0..=255).text(name));
                                    
                                    if value != &VRChatOSCType::Int(v) {
                                        *value = VRChatOSCType::Int(v);
                                        on_change(engine, name.clone(), value.clone()).unwrap();
                                    }
                                },
                            }
                        });
                    }
                });
            });
        });
    }
}

fn on_change(engine: &VRChatOSC, addr: String, value: VRChatOSCType) -> Result<(), Box<dyn Error>> {
    engine.send_vrc_input(vrchat_osc::VRChatOSCInput::Avatar(addr, value))?;

    Ok(())
}
