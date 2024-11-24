use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::plot::{Line, Plot, Value, Values};
use eframe::egui::{self, RichText};
use eframe::{self, App, Frame};
use serialport::SerialPortInfo;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Serial Data Plotter",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(MyApp::new())
        }),
    );
}

struct MyApp {
    available_ports: Vec<SerialPortInfo>,
    selected_port_index: Option<usize>,
    baud_rates: Vec<u32>,
    selected_baud_rate_index: usize,


    read_handle: Option<thread::JoinHandle<()>>,
    rx: Option<Receiver<f64>>,

    data: Vec<[f64; 2]>,
    start_time: Option<Instant>,
    is_collecting: bool,

    window_length: f64,
    y_min: f64,

    y_max: f64,


}

impl MyApp {
    pub fn new() -> Self {
        let ports = serialport::available_ports().unwrap_or_else(|_| Vec::new());

        let baud_rates = vec![
            110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200, 128000,
            256000,
        ];



        Self {
            available_ports: ports,
            selected_port_index: None,
            baud_rates,
            selected_baud_rate_index: 11,


            read_handle: None,
            rx: None,

            data: Vec::new(),
            start_time: None,
            is_collecting: false,

            window_length: 10.0,
            y_min:-100.00,

            y_max:1000.00,
        }
    }
    fn start_collection(&mut self) {
        if self.is_collecting {
            return;
        }

        if let Some(port_index) = self.selected_port_index {
            let port_info = self.available_ports[port_index].clone();
            let port_name = port_info.port_name.clone();
            let baud_rate = self.baud_rates[self.selected_baud_rate_index];

            let (tx, rx) = mpsc::channel();
            self.rx = Some(rx);

            self.data.clear();
            self.start_time = Some(Instant::now());
            self.is_collecting = true;

            self.read_handle = Some(thread::spawn(move || {
                let port_result = serialport::new(&port_name, baud_rate)
                    .timeout(Duration::from_millis(100))
                    .open();

                match port_result {
                    Ok(mut port) => {
                        let mut serial_buf: Vec<u8> = vec![0; 1024];
                        println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
                        loop {
                            match port.read(serial_buf.as_mut_slice()) {
                                Ok(t) => {
                                    let data = &serial_buf[..t];
                                    if let Ok(s) = std::str::from_utf8(data) {
                                        for line in s.lines() {
                                            if let Ok(value) = line.trim().parse::<f64>() {
                                                if tx.send(value).is_err() {
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(ref e)
                                    if e.kind() == std::io::ErrorKind::WouldBlock
                                        || e.kind() == std::io::ErrorKind::TimedOut =>
                                {
                                    thread::sleep(Duration::from_millis(10));
                                }
                                Err(e) => {
                                    eprintln!("{:?}", e);
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                    }
                }
            }));
        } else {
            println!("No serial port selected.");
        }
    }

    fn stop_collection(&mut self) {
        if self.is_collecting {
            self.rx = None;

            if let Some(handle) = self.read_handle.take() {
                handle.join().expect("Failed to join thread");
            }

            self.is_collecting = false;
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Label::new(
                    RichText::new("Serial Data Plotter").heading().strong(),
                ));
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.hyperlink_to("About","https://www.cloakycodes.me");
                    ui.
                    if ui.button("Refresh Ports").clicked() && !self.is_collecting {
                        self.available_ports =
                            serialport::available_ports().unwrap_or_else(|_| Vec::new());
                    }
                });
            });
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.heading("Configuration");

            ui.label("Select Serial Port:");

            if !self.is_collecting {
                egui::ComboBox::from_id_source("port_combo")
                    .selected_text(
                        self.selected_port_index
                            .and_then(|i| self.available_ports.get(i))
                            .map_or("Select a port", |info| info.port_name.as_str()),
                    )
                    .show_ui(ui, |ui| {
                        for (i, port) in self.available_ports.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.selected_port_index,
                                Some(i),
                                &port.port_name,
                            );
                        }
                    });

                ui.label("Select Baud Rate:");

                egui::ComboBox::from_id_source("baud_combo")
                    .selected_text(format!(
                        "{}",
                        self.baud_rates[self.selected_baud_rate_index]
                    ))
                    .show_ui(ui, |ui| {
                        for (i, &baud) in self.baud_rates.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.selected_baud_rate_index,
                                i,
                                format!("{}", baud),
                            );
                        }
                    });
                ui.separator();



            } else {
                if let Some(port_index) = self.selected_port_index {
                    if let Some(port_info) = self.available_ports.get(port_index) {
                        ui.label(format!("Port: {}", port_info.port_name));
                    }
                }
                ui.label(format!(
                    "Baud Rate: {}",
                    self.baud_rates[self.selected_baud_rate_index]
                ));
            };







            ui.separator();

            ui.add(egui::Slider::new(&mut self.window_length, 4.0..=100.0).text("Window Length"));





            ui.separator();

            ui.separator();

            if self.is_collecting {
                if ui.button("Stop").clicked() {
                    self.stop_collection();
                }
            } else {
                if ui.button("Start").clicked() {
                    self.start_collection();
                }
            }

            ui.label("Make sure your device is connected and sending data.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_collecting {
                if let Some(rx) = &self.rx {
                    while let Ok(value) = rx.try_recv() {
                        if let Some(start_time) = self.start_time {
                            let elapsed = start_time.elapsed().as_secs_f64();
                            self.data.push([elapsed, value]);

                            // Always keep only window_length worth of data
                            if self.data.len() > 2 {
                                // Need at least 2 points for a line
                                let latest_time = self.data.last().unwrap()[0];
                                let cutoff_time = latest_time - self.window_length;
                                while self.data.len() > 2 && self.data[0][0] < cutoff_time {
                                    self.data.remove(0);
                                }
                            }
                        }
                    }
                }
            }

            if !self.data.is_empty() {
                let line = Line::new(Values::from_values_iter(
                    self.data.iter().cloned().map(|[x, y]| Value::new(x, y)),
                ));

                let mut plot = Plot::new("Serial Data Plot")
                    .view_aspect(2.0)
                    .legend(egui::plot::Legend::default());

                // Always set x-axis range to follow the latest data
                let latest_time = self.data.last().unwrap()[0];
                plot = plot
                    .include_x(latest_time - self.window_length)
                    .include_x(latest_time).include_y(self.y_max).include_y(self.y_min);

                // if !self.auto_scale_y {
                //     plot = plot.include_y(self.y_min).include_y(self.y_max).include_x(latest_time - self.window_length);
                // }

                plot.show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
            } else {
                ui.label("No data to display. Start collection to see data.");
            }
        });

        ctx.request_repaint();
    }
}
