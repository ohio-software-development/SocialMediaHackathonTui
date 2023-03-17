// Rust TUI interface for mockup terminal social media site by @BP-2 and @jude-shreffler

use cursive::views::{Dialog, LinearLayout, TextView,EditView};
use cursive::theme::{Color, Theme, PaletteColor, BaseColor};
use cursive::{Cursive, CursiveExt, event, menu};
use cursive_extras::{*};
use std::fs;

mod image_view;


fn main() {
    let mut siv = Cursive::new();
    siv.set_theme(better_theme());
    
    // notes:
    // .child(EditView::new().content("blahblahblah"));
    
    // img = image_view::ImageView::set_image(&mut img, "IMG_7223[20].png");
    let _login_menu = Dialog::around(styled_editview("", "Login", true))
    .button("Enter", go_back_to_main_dialog)
    .button("Quit", |view| view.quit())
    .title("Login");
    // image
    // siv.add_layer(layout);
    siv.add_layer(_login_menu);
    

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
        friends_tree.add_leaf(friend, |s| swap_data(s, friend));
    }

    siv.menubar()
        .add_leaf("Home", go_back_to_main_dialog)
        .add_subtree("Friends", friends_tree);

}

fn swap_data(siv: &mut Cursive, name: &str) {
    siv.pop_layer();
    let file_path = "bios/".to_string() + name + ".bio";
    let bio = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    siv.add_layer(
        Dialog::around(TextView::new(bio))
            .title("Bio")
    );
}


fn go_back_to_main_dialog(siv: &mut Cursive) {

    let mut img = image_view::ImageView::new(30, 10);
    img.set_image("./download.jpeg");
    let image_viewer = Dialog::around(img);
    
    let layout = LinearLayout::vertical()
    .child(TextView::new("Profile:"))
    .child(image_viewer)
    .child(TextView::new("Bio:"))
    .child(TextView::new("Hi, my name is Brady and I am a sophomore studying\nComputer Science at Ohio University!"));


    // Remove the subdialog box
    siv.pop_layer();

    // clear the menu bar from the friends page
    siv.menubar().clear();
    siv.set_autohide_menu(true);
    siv.clear_global_callbacks(event::Key::Esc);

    // Show the main dialog box
    let _main_menu = Dialog::around(layout)
    .title("MyTui")
    .button("Browser", |s|s.quit())
    .button("Friends", open_friends)
    .button("Messages", |s|s.quit())
    .button("Edit", |s| s.quit())
    .button("Logout", |s| s.quit());

    // image
    
    siv.add_layer(_main_menu);
}

fn open_instagram(_: &mut Cursive){
    let path = "https://www.instagram.com/";
    match open::that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}
fn open_linkedin(_: &mut Cursive){
    let path = "https://www.linkedin.com/";
    match open::that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}
fn open_facebook(_: &mut Cursive){
    let path = "https://www.facebook.com/";
    match open::that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}







// todo: series of functions to display different UI menus for social media interface
// todo: image renderer?
// todo: put together presentation and polish idea?? (idk if they using dev post or how we are supposed to submit so idk ab this yet)