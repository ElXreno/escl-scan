extern crate gio;
extern crate gtk;
extern crate scan;

use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;
use gtk::{
    Application, ApplicationWindow, Builder, ComboBoxText, Entry, FileChooserAction,
    FileChooserDialog, FileFilter, MessageDialog, ResponseType, ToolButton,
};

fn main() {
    let application = Application::new(Some("com.elxreno.escl-scan.gui"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| build_ui(app));

    application.run(&[]);
}

fn build_ui(app: &Application) {
    let builder = Builder::new_from_string(include_str!("main.glade"));

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(app));

    let ip_entry: Entry = builder
        .get_object("ip_entry")
        .expect("Couldn't get ip_entry");

    let dpi_combo_box: ComboBoxText = builder
        .get_object("dpi_combo_box")
        .expect("Couldn't get dpi_combo_box");

    let scan_button: ToolButton = builder
        .get_object("scan_button")
        .expect("Couldn't get scan_button");

    let ip_is_empty_dialog: MessageDialog = builder
        .get_object("ip_is_empty_message_dialog")
        .expect("Couldn't get ip_is_empty_message_dialog");

    let scanning_dialog: MessageDialog = builder
        .get_object("scanning_progress_message_dialog")
        .expect("Couldn't get scanning_progress_message_dialog");

    scan_button.connect_clicked(clone!(@weak window => move |_| {
        let ip = ip_entry.get_text().unwrap();
        let scan_resolution = dpi_combo_box.get_active_text().unwrap();

        if ip.is_empty() {
            ip_is_empty_dialog.run();
            ip_is_empty_dialog.hide();
        } else {
            let file_chooser = FileChooserDialog::new(
                Some("Save File"),
                Some(&window),
                FileChooserAction::Save,
            );

            file_chooser.set_do_overwrite_confirmation(true);

            let filter = FileFilter::new();
            filter.set_name(Some("JPG"));
            filter.add_mime_type("image/jpeg");

            file_chooser.add_filter(&filter);
            file_chooser.set_current_name("scan.jpg");

            file_chooser.add_buttons(&[
                ("Save", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ]);

            if file_chooser.run() == ResponseType::Cancel {
                return;
            }

            let destination_file = file_chooser.get_filename().expect("Couldn't get filename");

            file_chooser.destroy();

            println!(
                "IP: {}; DPI: {}; Output file: {}",
                if ip.is_empty() { "empty" } else { ip.as_str() },
                scan_resolution,
                destination_file.display()
            );

            let scanner_base_path = format!("http://{}:80/eSCL", ip);

//            scanning_dialog.run();

            scan::scan(
                scanner_base_path.as_str(),
                scan_resolution.parse::<i16>().unwrap(),
                destination_file.to_str().unwrap()
            );

//            scanning_dialog.hide();
        }
    }));

    window.show_all();
}