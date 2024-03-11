#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release
use libmem::*;
use eframe::egui::{self, Color32, FontFamily::Monospace, FontId, Layout};

mod injector;

fn main() {
    let _ = std::fs::DirBuilder::new().create(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 140.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );
}


struct MyApp {
    injected: bool,
    era_process: lm_process_t,
    engine_module: lm_module_t,
    control_module: Option<lm_module_t>,
    deflate_address: usize,
    deflate_hook_address: Option<usize>,
    deflate_trampoline: Option<(usize, usize)>,
    inflate_address: usize,
    inflate_hook_address: Option<usize>,
    inflate_trampoline: Option<(usize, usize)>,
    status: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().text_styles.extend([
                    (egui::TextStyle::Button, FontId::new(24., Monospace)),
                    (egui::TextStyle::Body, FontId::new(24., Monospace))
                ].into_iter());
    
                if !self.injected {
                    if ui.add(egui::Button::new("Inject")).clicked() {
                        LM_LoadModuleEx(&self.era_process, "./target/debug/GameControl.dll");//.expect("Failed to load GameControl module.");
                        self.control_module = LM_FindModuleEx(&self.era_process, "GameControl.dll");
        
                        self.deflate_hook_address = LM_FindSymbolAddress(&self.control_module.expect("Failed to find GameControl module."), "deflate");
                        self.deflate_trampoline = LM_HookCodeEx(&self.era_process, self.deflate_address, self.deflate_hook_address.expect("Failed to find deflate hook symbol."));
                        let deflate_return = LM_FindSymbolAddress(&self.control_module.expect("Failed to find GameControl module."), "DEFLATE_RETURN").expect("Failed to find deflate return symbol");
                        LM_WriteMemoryEx(&self.era_process, deflate_return, &self.deflate_trampoline.unwrap().0).unwrap();
                        
        
                        // HOOKING INFLATE WILL NOT WORK :(
                            
                        //self.inflate_hook_address = LM_FindSymbolAddress(&self.control_module.expect("Failed to find GameControl module."), "inflateEnd");
                        //self.inflate_trampoline = LM_HookCodeEx(&self.era_process, self.inflate_address+2, self.inflate_hook_address.expect("Failed to find inflate hook symbol."));
                        //let inflate_return = LM_FindSymbolAddress(&self.control_module.expect("Failed to find GameControl module."), "INFLATE_RETURN").expect("Failed to find inflate return symbol");
                        //
                        //self.inflate_trampoline = self.inflate_trampoline.map(|(addr, size)| (addr-2, size+2));
                        //LM_WriteMemoryEx(&self.era_process, self.inflate_address, &0x9090u16).unwrap(); //replace `push rbx` with nops
                        //LM_WriteMemoryEx(&self.era_process, self.inflate_trampoline.unwrap().0, &0x3504u16); //Write `push rbx` at the beginning of the trampoline;
                        //LM_WriteMemoryEx(&self.era_process, inflate_return, &self.inflate_trampoline.unwrap().0).unwrap(); //write the trampoline address to the inflate return 
        
                        self.status = "Function successfully hooked".to_string();
                        self.injected = true;
                    }
                } else {
                    if ui.add(egui::Button::new("Eject")).clicked() {
                        //unhook the deflate function
                        //copy byte by byte because of rust type sizing constraints
                        let tramp = self.deflate_trampoline.take().unwrap();
                        for offset in 0..tramp.1 {
                            let trampoline_data = LM_ReadMemoryEx::<u8>(&self.era_process, tramp.0 + offset).unwrap();
                            LM_WriteMemoryEx(&self.era_process, self.deflate_address+offset, &trampoline_data).unwrap();
                        }
        
                        LM_UnloadModuleEx(&self.era_process, &self.control_module.take().expect("Expected to find control module")).expect("Failed to unload module");
        
                        self.deflate_hook_address = None;
                        self.injected = false;
        
                        let default_file_dir = std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/";
                        if let Some(filename) = rfd::FileDialog::new()
                            .add_filter("Comma Separated Values", &["CSV"])
                            .set_directory(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/")
                            .save_file() {
                                let mut dir = filename.parent().map(|p| String::from(p.to_str().unwrap())).unwrap();
                                dir.push('\\');
                                let stem = filename.file_stem().map_or(String::from(""), |s| String::from(s.to_str().unwrap_or("")));
        
                                //let ingress = dir.clone() + &stem.clone() + "_ingress.csv";
                                let egress = dir + &stem + "_egress.csv";
                                
                                //std::fs::rename(default_file_dir.clone()+"in.csv", ingress).unwrap_or_else(|e| self.status = e.to_string());
                                std::fs::rename(default_file_dir+"out.csv", egress).unwrap_or_else(|e| self.status = e.to_string());
                            }
                    }
                }

                ui.colored_label(if self.injected { Color32::GREEN } else { Color32::RED }, if self.injected { "injected "} else { "not injected" });

            });

            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                let file_len = std::fs::File::open(std::env::var("USERPROFILE").unwrap() + "/Documents/GShovel/tmp/out.csv")
                    .map(|f| f.metadata().map_or(0, |md| md.len())).unwrap_or(0);
                ui.label(format!("{file_len}"));
            });
        });

    }
}
impl Default for MyApp {
    fn default() -> Self {
        let era_process = LM_FindProcess("Era.exe").expect("Failed to find Graal process.");
        let engine_module = LM_FindModuleEx(&era_process, "Graal3DEngine.dll").expect("Failed to find engine module.");
        let deflate_address = LM_FindSymbolAddress(&engine_module, "deflate").expect("Failed to find deflate symbol to hook.");
        let inflate_address = LM_FindSymbolAddress(&engine_module, "inflateEnd").expect("Failed to find inflate symbol to hook.");

        Self { 
            injected: Default::default(),
            era_process,
            engine_module,
            control_module: Default::default(),
            deflate_address,
            deflate_hook_address: Default::default(),
            deflate_trampoline: Default::default(),
            inflate_address,
            inflate_hook_address: Default::default(),
            inflate_trampoline: Default::default(),
            status: Default::default(),
        }
    }
}
