use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

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

		win.show_all();

	});

	app.run();
}
