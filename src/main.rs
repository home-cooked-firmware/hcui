use std::error::Error;
use gilrs::{Event, EventType, Gilrs};
use slint::{SharedString, Timer, TimerMode, Weak};
use std::time::Duration;

slint::include_modules!();

pub fn poll(gilrs: &mut Gilrs, app: &Weak<MainWindow>) {
    if let Some(app) = app.upgrade() {
        while let Some(Event { id, event, time, .. }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                EventType::ButtonPressed(button, code) => {
                    println!("{:?} - {:?}", button, code);

                    app.invoke_button(SharedString::from(format!("{}", code.into_u32())));
                },
                _ => {

                }
            }
            //app.invoke_button(SharedString::from(event));
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let screen_width = std::env::var("HCUI_SCREEN_WIDTH").unwrap_or("640".to_string());
    let screen_height = std::env::var("HCUI_SCREEN_HEIGHT").unwrap_or("480".to_string());

    let mut gilrs = Gilrs::new().unwrap();

    let ui = MainWindow::new()?;
    let ui_weak = ui.as_weak();

    let poll_timer = Timer::default();

    poll_timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
        poll(&mut gilrs, &ui_weak);
    });

    ui.set_screen_width(screen_width.parse::<i32>().unwrap());
    ui.set_screen_height(screen_height.parse::<i32>().unwrap());

    // ui.on_button(|| {
    //     println!("DFDSF");
    // });

    ui.show().unwrap();
    slint::run_event_loop().unwrap();
    //ui.run()?;

    Ok(())
}
