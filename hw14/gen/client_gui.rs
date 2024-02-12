use std::ops::Index;

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use rand::Rng;
use reqwest::StatusCode;
use restapi_smarthouse::{
    smartdevice::SmartDeviceUpdate, smartroom::SmartRoom, smartsocket::SmartSocketUpdate,
};

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GUI Client of SmartHouse",
        native_options,
        Box::new(|cc| Box::new(GuiClientApp::new(cc))),
    )
}

#[derive(Debug, Clone)]
struct SmartGrid {
    _idx: usize,
    room: String,
    _room_id: i64,
    device: String,
    device_id: i64,
    device_type: String,
    report: String,
}

#[derive(Default)]
struct GuiClientApp {
    err: String,
    addr: String,
    connected: bool,
    rooms: Vec<SmartRoom>,
    row_idx_selected: usize,
    rooms2: Vec<SmartGrid>,
}

impl GuiClientApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        GuiClientApp::default()
    }
    fn connect(&mut self) {
        self.connected = false;
        let url = format!("{}rooms", self.addr);

        let res = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let response = reqwest::get(url).await;
            if response.is_err() {
                self.err = response.unwrap_err().to_string();
                return Err(self.err.clone());
            };

            let text = response.unwrap().text().await;
            if text.is_err() {
                self.err = text.unwrap_err().to_string();
                return Err(self.err.clone());
            }

            self.rooms = serde_json::from_str(text.unwrap().as_ref()).unwrap();
            self.rooms2.clear();
            let mut idx = 0;
            for room in self.rooms.iter() {
                for device in room.devices.iter() {
                    let sg = SmartGrid {
                        _idx: idx,
                        room: room.name.to_string(),
                        _room_id: room.id,
                        device: device.get_name().to_string(),
                        device_id: device.get_id(),
                        device_type: device.get_type().to_string(),
                        report: device.get_report(),
                    };
                    self.rooms2.push(sg);
                    idx += 1;
                }
            }
            self.connected = true;
            Ok(())
        });
        println!("res:{:?}", res);
    }

    fn _action_on(&mut self, _sd: SmartGrid) {
        let res = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let client = reqwest::Client::new();
            let url = format!("{}devices/{}", self.addr, _sd.device_id);
            let sdu = SmartDeviceUpdate::Socket(SmartSocketUpdate {
                state: Some(true),
                power: Some(rand::thread_rng().gen_range(0.01..220.00)),
            });

            let response = client.put(url).json(&sdu).send().await.unwrap();
            let code = response.status();

            if code != StatusCode::OK {
                return Err(":500");
            }

            let text = response.text().await.unwrap();
            println!("Device has been updated: {}", text);

            Ok(())
        });
        println!("Action socket On:{:?}", res);
    }
    fn _action_off(&mut self, _sd: SmartGrid) {
        let res = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let client = reqwest::Client::new();
            let url = format!("{}devices/{}", self.addr, _sd.device_id);
            let sdu = SmartDeviceUpdate::Socket(SmartSocketUpdate {
                state: Some(false),
                power: Some(0.0),
            });

            let response = client.put(url).json(&sdu).send().await.unwrap();
            let code = response.status();

            if code != StatusCode::OK {
                return Err(":500");
            }

            let text = response.text().await.unwrap();
            println!("Device has been updated: {}", text);

            Ok(())
        });
        println!("Action socket Off:{:?}", res);
    }

    fn toggle_row_selection(&mut self, sg: &SmartGrid, _row_response: &egui::Response) {
        if _row_response.clicked() {
            self.row_idx_selected = sg._idx;
        }
    }
}

impl eframe::App for GuiClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Address to connect:");
                    self.addr = "http://0.0.0.0:8089/".to_string();
                    ui.text_edit_singleline(&mut self.addr);
                    let button_text = if self.connected {
                        "Connected".to_string()
                    } else {
                        format!("Connect.{}", self.err)
                    };
                    if ui.button(button_text).clicked() {
                        self.connect()
                    };
                    if !self.rooms2.is_empty() {
                        let item = self.rooms2.index(self.row_idx_selected).clone();
                        if item.device_type == "socket" {
                            if item.report.contains("Off") {
                                ui.horizontal(|ui| {
                                    if ui.button("On").clicked() {
                                        self._action_on(item);
                                        self.connect();
                                    }
                                });
                            } else {
                                ui.horizontal(|ui| {
                                    if ui.button("Off").clicked() {
                                        self._action_off(item);
                                        self.connect();
                                    }
                                });
                            }
                        }
                    }
                    if self.connected && ui.button("Refresh").clicked() {
                        self.connect()
                    }
                });
                ui.separator();
                ui.heading("Control panel of SmartHouse:");
                ui.separator();
                ui.vertical(|ui| {
                    let mut table = TableBuilder::new(ui)
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::remainder())
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .min_scrolled_height(0.0);
                    table = table.sense(egui::Sense::click());

                    table
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Room");
                            });
                            header.col(|ui| {
                                ui.strong("Device");
                            });
                            header.col(|ui| {
                                ui.strong("State");
                            });
                        })
                        .body(|mut body| {
                            let mut row_index = 0;
                            for item in self.rooms2.clone() {
                                body.row(20.0, |mut row| {
                                    row.set_selected(self.row_idx_selected == row_index);
                                    row.col(|ui| {
                                        ui.label(item.room.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(item.device.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(item.report.to_string());
                                    });
                                    self.toggle_row_selection(&item, &row.response());
                                    row_index += 1;
                                });
                            }
                        });
                });
            });
        });
    }
}
