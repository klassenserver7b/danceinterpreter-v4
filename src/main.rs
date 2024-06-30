//pub mod dataloading;

slint::include_modules!();
fn main() {
    println!("Hello, world!");

    MainWindow::new().unwrap().run().unwrap();
}