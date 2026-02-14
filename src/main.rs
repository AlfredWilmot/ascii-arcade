use ascii_arcade::{
    app::{App, Mode, State},
    scene,
    ui::ui,
    user_input,
};

fn main() -> std::io::Result<()> {
    let mut terminal = scene::init().expect("ERROR: could not setup terminal!");
    let rx = user_input::create_data_channel();
    let mut app = App::new(Mode::Default);

    'menu: loop {
        terminal.draw(|frame| {
            ui(frame, &app);
        })?;

        match app.state {
            // clear the terminal if previously playing a game
            State::Playing(_) => {
                _ = terminal.clear();
            }
            State::Exit => break 'menu,
            _ => {}
        }
        app.update(&rx);
    }

    scene::close(&mut terminal);
    Ok(())
}
