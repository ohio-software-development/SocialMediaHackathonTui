// Rust TUI interface for mockup terminal social media site by @BP-2 and @jude-shreffler

use cursive::views::{Dialog, TextView,EditView};
use cursive::theme::{Color, Theme, PaletteColor, BaseColor};
use cursive::{Cursive, CursiveExt, event, menu};
use std::fs;

fn main() {
    let mut siv = Cursive::new();
    let mut theme = Theme::default();

    theme.palette[PaletteColor::Background] = Color::Rgb(21, 71, 52);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::Shadow] = Color::Light(BaseColor::White);
    

    siv.set_theme(theme);
    
    go_back_to_main_dialog(&mut siv);

    siv.add_global_callback('q', |s| s.quit());
    

    siv.run();
}

fn open_friends(siv: &mut Cursive)
{
    siv.pop_layer();

    siv.set_autohide_menu(false);
    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());

    // create a tree of friends
    let friends = vec!["Brady Phelps", "Alex Bikowski", "Gerald Yurek"];
    let mut friends_tree = menu::Tree::new();
    for friend in friends {
        friends_tree.add_leaf(friend, swap_data);
    }


    siv.menubar()
        .add_leaf("Home", go_back_to_main_dialog)
        .add_leaf("Next", go_back_to_main_dialog)
        .add_leaf("Previous", go_back_to_main_dialog)
        .add_subtree("Friends", friends_tree);

}

fn swap_data(siv: &mut Cursive) {

}

fn go_back_to_main_dialog(siv: &mut Cursive) {
    // Remove the subdialog box
    siv.pop_layer();

    // clear the menu bar from the friends page
    siv.menubar().clear();
    siv.set_autohide_menu(true);
    siv.clear_global_callbacks(event::Key::Esc);

    // Show the main dialog box
    let _main_menu = Dialog::new()
    .title("MyTui")
    .button("Browser", |s|s.quit())
    .button("Friends", open_friends)
    .button("Messages", |s|s.quit())
    .button("Edit", |s| s.quit())
    .button("Logout", |s| s.quit());

    // image
    



    siv.add_layer(_main_menu);
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