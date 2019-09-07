extern crate client;
extern crate gio;
extern crate gtk;

use std::error::Error;
use std::process::exit;

use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gtk::GtkWindowExt;
use gtk::WidgetExt;

use client::gui::Gui;

fn main() -> Result<(), Box<dyn Error>>
{
    gtk::init()?;
    let application = gtk::Application::new(None, gio::ApplicationFlags::empty())?;
    application.connect_activate(on_application_activate);
    let status = application.run(&[]);
    exit(status);
}

fn on_application_activate(application: &gtk::Application)
{
    let gui = Gui::new();
    gui.window.set_application(Some(application));
    gui.window.show_all();
}
