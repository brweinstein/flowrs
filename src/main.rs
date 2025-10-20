use flowrs::app::App;
use flowrs::tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    tui::run(&mut app)?;
    Ok(())
}
