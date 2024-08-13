use eframe::{run_native, App, NativeOptions};
use three_d::*;

fn main() {
    let app = MyApp {
        context: HeadlessContext::new().unwrap(),
    };
    let win_options = NativeOptions::default();

    println!("Running!");
    let result = run_native(
        "Node Graph",
        win_options,
        Box::new(|context| {
            println!("Creating renderer!");
            Ok(Box::new(app))
        }),
    );
    println!("Result: {:?}", result);
}

struct MyApp {
    context: HeadlessContext,
}

impl App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        println!("Updating!");
    }
}
