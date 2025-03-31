use ascii_arcade::{
    app::{App, Mode, State},
    scene,
    ui::ui,
    user_input,
};

fn main() {
    let mut terminal = scene::init().expect("ERROR: could not setup terminal!");
    let rx = user_input::create_data_channel();
    let mut app = App::new(Mode::Default);

    'menu: loop {
        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .expect("ERROR: could not draw frame!");

        // block updating the main-menu between user-input events,
        // much easier on the cpu than a rx.try_rec() + thread::sleep()
        if let Ok(event) = rx.recv() {
            // update the app based on the event, and handle the new state
            if let State::Exit = app.update(event) {
                break 'menu;
            };
        }
    }

    scene::close(terminal);
}
