// Rust TUI interface for mockup terminal social media site by @BP-2 and @jude-shreffler

use cursive::views::{Dialog, TextView,EditView};
use cursive::theme::{Color, Theme, PaletteColor, BaseColor};
use cursive::{Cursive, CursiveExt};
use std::fs;


fn main() {
    let mut siv = Cursive::new();
    let mut theme = Theme::default();

    theme.palette[PaletteColor::Background] = Color::Rgb(0, 104, 72);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::Shadow] = Color::Light(BaseColor::White);
    

    siv.set_theme(theme);
    

    let _main_menu = Dialog::new()
        .title("Menu")
        .content(TextView::new("Blackboard Rust TUI"))
        .button("Login", |s| s.quit())
        .button("Team Members", open_subdialog)
        .button("Quit", |s| s.quit())
        .button("Display File", open_file);

    siv.add_layer(_main_menu);

    siv.add_global_callback('q', |s| s.quit());
    

    siv.run();
}

fn open_subdialog(siv: &mut Cursive)
{
    siv.pop_layer();

    siv.add_layer(
        Dialog::new()
            .title("Team Members")
            .content(TextView::new("Brady Phelps\nMichael Tan\nnPreston Rembis\nAlex Bikowski"))
            .button("Back", go_back_to_main_dialog),
    );
}

fn go_back_to_main_dialog(siv: &mut Cursive) {
    // Remove the subdialog box
    siv.pop_layer();

    // Show the main dialog box
    siv.add_layer(
        Dialog::new()
            .title("Menu")
            .content(TextView::new("Blackboard Rust TUI"))
            .button("Login", |s| s.quit())
            .button("Team Members", open_subdialog)
            .button("Quit", |s| s.quit())
            .button("Display File", open_file)
    );
}

fn open_file(siv: &mut Cursive) {
    
    siv.pop_layer();


    let contents = fs::read_to_string("hello.txt")
        .expect("Should have been able to read the file");
    

    siv.add_layer(
        Dialog::new()
        .title("input.txt")
        .content(TextView::new(contents))
        .button("Back", go_back_to_main_dialog)
    );
}



// todo: series of functions to display different UI menus for social media interface
// todo: image renderer?
// todo: put together presentation and polish idea?? (idk if they using dev post or how we are supposed to submit so idk ab this yet)