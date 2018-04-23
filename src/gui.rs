extern crate gtk;
extern crate gdk_pixbuf;

use relm::{Relm, Update, Widget};
use gtk::Orientation::{Vertical, Horizontal};

// Widget
use gtk::{
    Window,
    WindowType,
    GtkWindowExt,
    WidgetExt,
    Button,
    ButtonExt,
    ContainerExt,
    Inhibit,
    FileChooserButton,
    FileChooserAction,
    FileChooserButtonExt,
    Image,
    ImageExt,
};


use self::gdk_pixbuf::Pixbuf;

pub struct Model {

}

pub struct Widgets {
    exit_button: Button,
    filechooser_button: FileChooserButton,
    window: Window,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Choose_file,
}

pub struct Win {
    model: Model,
    widgets: Widgets,
}


impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }
    
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::Choose_file => println!("HIHIHIH")
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

        // Create a box
        let vbox = gtk::Box::new(Vertical, 0);
        // let vboxB = gtk::Box::new(Vertical, 0);
        // Add botton into box
        let exit_button = Button::new_with_label("Exit");
        let filechooser_button = FileChooserButton::new("Select", FileChooserAction::Open);
        let window = Window::new(WindowType::Toplevel);

        let img_pixbuf = Pixbuf::new_from_file_at_size("data/input.bmp", 500, 500).unwrap();
        let img = Image::new_from_pixbuf(&img_pixbuf);
        // let img = Image::new_from_file("data/input.bmp");
        // img.set_pixel_size(10);
        vbox.add(&img);
        vbox.add(&filechooser_button);
        vbox.add(&exit_button);
        // window.add(&vboxB);
        window.add(&vbox);
        // window.fullscreen();

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, exit_button, connect_clicked(_), Msg::Quit);
        // TODO WHYYYYYY
        connect!(relm, filechooser_button, connect_file_set(_), Msg::Choose_file);
        
        // TODO: find out the which stucture contains this fucking method
        window.show_all();
        
        // return
        Win {
            model,
            widgets: Widgets {
                exit_button,
                filechooser_button,
                window: window,
            }
        }
    }
}
