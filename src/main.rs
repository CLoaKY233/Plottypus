use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, RichText};
use eframe::egui::plot::{Line, Plot, Value, Values};
use eframe::{self, App, Frame};
use serialport::SerialPortInfo;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Serial Data Plotter",
        native_options,
        Box::new(|cc| {
            // Apply dark theme
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(MyApp::new())
        }),
    );
}

struct MyApp {
    // Serial port configuration
    available_ports: Vec<SerialPortInfo>,
    selected_port_index: Option<usize>,
    baud_rates: Vec<u32>,
    selected_baud_rate_index: usize,

    // Serial communication
    read_handle: Option<thread::JoinHandle<()>>,
    rx: Option<Receiver<f64>>,

    // Plotting data
    data: Vec<[f64; 2]>,
    start_time: Option<Instant>,
    is_collecting: bool,
}

impl MyApp {
    pub fn new() -> Self {
        // Get the list of available serial ports
        let ports = serialport::available_ports().unwrap_or_else(|_| Vec::new());

        // Predefined list of common baud rates
        let baud_rates = vec![
            110, 300, 600, 1200, 2400, 4800, 9600,
            14400, 19200, 38400, 57600, 115200, 128000, 256000,
        ];

        Self {
            available_ports: ports,
            selected_port_index: None,
            baud_rates,
            selected_baud_rate_index: 6, // Default to 9600 baud

            read_handle: None,
            rx: None,

            data: Vec::new(),
            start_time: None,
            is_collecting: false,
        }
    }

    fn start_collection(&mut self) {
        if self.is_collecting {
            return;
        }

        // Ensure a port is selected
        if let Some(port_index) = self.selected_port_index {
            let port_info = self.available_ports[port_index].clone();
            let port_name = port_info.port_name.clone();
            let baud_rate = self.baud_rates[self.selected_baud_rate_index];

            // Create a new channel
            let (tx, rx) = mpsc::channel();
            self.rx = Some(rx);

            self.data.clear();
            self.start_time = Some(Instant::now());
            self.is_collecting = true;

            // Spawn a thread to read from the serial port
            self.read_handle = Some(thread::spawn(move || {
                let port_result = serialport::new(&port_name, baud_rate)
                    .timeout(Duration::from_millis(100)) // Set a small timeout
                    .open();

                match port_result {
                    Ok(mut port) => {
                        let mut serial_buf: Vec<u8> = vec![0; 1024];
                        println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
                        loop {
                            match port.read(serial_buf.as_mut_slice()) {
                                Ok(t) => {
                                    let data = &serial_buf[..t];
                                    // Process your data here and extract the value you want to plot
                                    if let Ok(s) = std::str::from_utf8(data) {
                                        for line in s.lines() {
                                            if let Ok(value) = line.trim().parse::<f64>() {
                                                if tx.send(value).is_err() {
                                                    // Receiver has been dropped
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock
                                    || e.kind() == std::io::ErrorKind::TimedOut =>
                                {
                                    // No data available right now
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
            // No port selected
            println!("No serial port selected.");
        }
    }

    fn stop_collection(&mut self) {
        if self.is_collecting {
            // Dropping the rx channel will signal the thread to stop
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
        // Create the GUI layout
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Label::new(
                    RichText::new("Serial Data Plotter").heading().strong(),
                ));
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("Refresh Ports").clicked() && !self.is_collecting {
                        // Refresh the list of available ports
                        self.available_ports = serialport::available_ports().unwrap_or_else(|_| Vec::new());
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Configuration");

            // Serial port selection
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

                // Baud rate selection
                ui.label("Select Baud Rate:");

                egui::ComboBox::from_id_source("baud_combo")
                    .selected_text(format!("{}", self.baud_rates[self.selected_baud_rate_index]))
                    .show_ui(ui, |ui| {
                        for (i, &baud) in self.baud_rates.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.selected_baud_rate_index,
                                i,
                                format!("{}", baud),
                            );
                        }
                    });
            } else {
                // Show selected port and baud rate (read-only)
                if let Some(port_index) = self.selected_port_index {
                    if let Some(port_info) = self.available_ports.get(port_index) {
                        ui.label(format!("Port: {}", port_info.port_name));
                    }
                }
                ui.label(format!("Baud Rate: {}", self.baud_rates[self.selected_baud_rate_index]));
            }

            ui.separator();

            // Start/Stop buttons
            if self.is_collecting {
                if ui.button("Stop").clicked() {
                    self.stop_collection();
                }
            } else {
                if ui.button("Start").clicked() {
                    self.start_collection();
                }
            }

            ui.separator();

            // Display some info or instructions
            ui.label("Make sure your device is connected and sending data.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Update data
            if self.is_collecting {
                // Receive data from the serial port
                if let Some(rx) = &self.rx {
                    while let Ok(value) = rx.try_recv() {
                        if let Some(start_time) = self.start_time {
                            let elapsed = start_time.elapsed().as_secs_f64();
                            self.data.push([elapsed, value]);
                            // Keep only the last N data points to prevent memory growth
                            if self.data.len() > 1000 {
                                self.data.drain(0..(self.data.len() - 1000));
                            }
                        }
                    }
                }
            }

            // Plotting
            if !self.data.is_empty() {
                let line = Line::new(Values::from_values_iter(
                    self.data.iter().cloned().map(|[x, y]| Value::new(x, y)),
                ));
                Plot::new("Serial Data Plot")
                    .view_aspect(2.0)
                    .legend(egui::plot::Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            } else {
                ui.label("No data to display. Start collection to see data.");
            }
        });

        ctx.request_repaint(); // Keep the UI updating
    }
}
