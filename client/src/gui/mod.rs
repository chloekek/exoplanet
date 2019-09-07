use gtk::BuilderExtManual;
use gtk::EntryExt;

pub struct Gui
{
    pub window: gtk::ApplicationWindow,
    pub command: gtk::Entry,
    pub log: gtk::TextView,
}

impl Gui
{
    pub fn new() -> Self
    {
        let glade = include_str!("mod.glade");
        let builder = gtk::Builder::new_from_string(glade);

        let window  = builder.get_object("window").unwrap();
        let command = builder.get_object("command").unwrap();
        let log     = builder.get_object("log").unwrap();

        Self{window, command, log}.initialize()
    }

    fn initialize(self) -> Self
    {
        self.command.connect_activate(Self::on_command_activate);
        self
    }

    fn on_command_activate(command: &gtk::Entry)
    {
        let text = command.get_text();
        command.set_text("");
        println!("{:?}", text);
    }
}
