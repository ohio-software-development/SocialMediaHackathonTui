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
    

    siv.set_autohide_menu(false);
    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());
    siv.add_global_callback('q', |s| s.quit());
    

    siv.run();
}

fn go_back_to_main_dialog(siv: &mut Cursive) {

    let mut img = image_view::ImageView::new(30, 10);
    img.set_image("./images/Brady Phelps.jpeg");
    let image_viewer = Dialog::around(img);
    
    let layout = LinearLayout::vertical()
    .child(TextView::new("Profile:"))
    .child(image_viewer)
    .child(TextView::new("Bio:"))
    .child(TextView::new("Hi, my name is Brady and I am a sophomore studying\nComputer Science at Ohio University!"));


    // Remove the subdialog box
    siv.pop_layer();

    // create the menu bar
    siv.menubar().clear();

    // create a subtree of friends
    let friends = vec!["Jude Shreffler", "Alex Bikowski", "Gerald Yurek"];
    let mut friends_tree = menu::Tree::new();
    for friend in friends {
        friends_tree.add_leaf(friend, |s| swap_data(s, friend));
    }

    siv.menubar()
        .add_leaf("Home", go_back_to_main_dialog)
        .add_subtree("Browser", menu::Tree::new()
            .leaf("Instagram", open_instagram)
            .leaf("Linkedin", open_linkedin)
            .leaf("Facebook", open_facebook)
        )
        .add_subtree("Friends", friends_tree)
        .add_leaf("Edit", go_back_to_main_dialog)
        .add_leaf("Logout", |s| s.quit());

    // Show the main dialog box
    let _main_menu = Dialog::around(layout)
    .title("MyTui");

    // image
    siv.add_layer(_main_menu);
}

fn swap_data(siv: &mut Cursive, name: &str) {
    siv.pop_layer();
    let file_path = "bios/".to_string() + name + ".bio";
    let bio = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut img = image_view::ImageView::new(30, 10);
    img.set_image("./download.jpeg");
    let image_viewer = Dialog::around(img);

    let layout = LinearLayout::vertical()
    .child(TextView::new("Profile:"))
    .child(image_viewer)
    .child(TextView::new("Bio:"))
    .child(TextView::new(bio));

    // Show the main dialog box
    let content = Dialog::around(layout)
    .title("MyTui");

    // image
    siv.add_layer(content);
}


fn edit_bio(siv: &mut Cursive){
    let mut img = image_view::ImageView::new(30, 10);
    img.set_image("./download.jpeg");
    let image_viewer = Dialog::around(img);
    
    let layout = LinearLayout::vertical()
    .child(TextView::new("Profile Editor:"))
    .child(image_viewer)
    .child(TextView::new("Bio:"))
    .child(EditView::new().on_submit(save_bio).content("Hi, my name is Brady and I am a sophomore studying\nComputer Science at Ohio University!"));
    // Remove the subdialog box
    siv.pop_layer();

    // clear the menu bar from the friends page
    siv.menubar().clear();
    siv.set_autohide_menu(true);
    siv.clear_global_callbacks(event::Key::Esc);

    // Show the main dialog box
    let edit_layer = Dialog::around(layout).button("Back", go_back_to_main_dialog);
    siv.add_layer(edit_layer);
}


fn save_bio(_: &mut Cursive, x: &str){
    let data = x;
    fs::write("./bios/Brady Phelps.bio", data).expect("Unable to write file");
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