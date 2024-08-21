use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {

    let app = Application::builder()
	.application_id("org.example.HelloWorld")
	.build();

    app.connect_activate(|app| {
	let win = ApplicationWindow::builder()
	    .application(app)
	    .default_width(320)
	    .default_height(200)
	    .title("Hello again, now with gtk3!")
	    .build();

	let btn = Button::with_label("Click here");
	btn.connect_clicked(|_| {
	    eprintln!("Wrong button!");
	});

	let other_btn = Button::with_label("Click here");
	other_btn.connect_clicked(|_| {
	    eprintln!("That's right!");
	});

	
	win.add(&btn);
	win.add(&other_btn);
	win.show_all();
  });
    app.run();
}
