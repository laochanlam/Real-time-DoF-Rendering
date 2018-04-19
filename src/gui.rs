extern crate gtk;

use relm::{Relm, Update, Widget};
use gtk::Orientation::Vertical;

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
};

pub struct Model {

}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Win {
    model: Model,
    window: Window,
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
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

        // Create a box
        let vbox = gtk::Box::new(Vertical, 0);
        // Add botton into box
        let exit_button = Button::new_with_label("Exit");
        vbox.add(&exit_button);

        let window = Window::new(WindowType::Toplevel);
        window.fullscreen();
        window.add(&vbox);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, exit_button, connect_clicked(_), Msg::Quit);
        // TODO: find out the which stucture contains this fucking method
        window.show_all();
        
        // return
        Win {
            model,
            window: window,
        }
    }
}
