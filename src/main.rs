use std::io;
use colored::*;

fn main() {
    clear();
    logo();
    mm();
}
// clear lines
fn clear() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
        clear
        exit 0
        "#
    )
    .unwrap();
    
    println!("{}", output);
}
// logos
fn logo() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/logo.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}
fn logouninstall() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/logouninst.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}

// main menu
fn mm() {

    let mut wtd = String::new();

    println!("\n\n{} {} {} {} {} {} {} {} {} {}", " 1)".cyan(), "DISTROS".yellow(), "2)".cyan(), "MyOS\n\n".yellow(), "3)".cyan(), "INFO ".yellow(),      
   "  4)".cyan() ,"UNINSTALL\n\n".yellow(), "0)".cyan(), "QUIT\n\n".red().bold());

   eprint!(" Coose an action: ");
   
    // menu match logic
    match io::stdin().read_line(&mut wtd) {
        Ok(_) => {},
        Err(_e) => println!("Ошибка программы!")
    }

    mm_logic(&wtd);
}
fn mm_logic(wtd: &str) {
    match wtd.trim() {
        "0" => println!("{}", "\n EXIT".red().bold()),
        "1"   => distros(),
        "2"   => myos(),
        "3"   => info(),
        "4"   => uninstall(),
        _e => clsmm(),
    }
}

// distros
fn distros() {
    clear();
    logo();
    println!("\n\n{} {} {} {} {} {} {} {} {} {}", " 1)".cyan(), "Solus".blue(), "2)".cyan(), "Fedora\n\n".white(), "3)".cyan(), "Manjaro ".green(),      
   "\n\n 5)".cyan() ,"Back".yellow(), "0)".cyan(), "QUIT\n\n".red().bold());

   let mut distro = String::new();

   eprint!(" Coose a distro: ");
   
   // menu match logic
   match io::stdin().read_line(&mut distro) {
       Ok(_) => {},
       Err(_e) => println!("Ошибка программы!")
   }

   distro_log(&distro);

}
fn distro_log(wtd: &str) {
    match wtd.trim() {
        "0" => println!("{}", "\n EXIT".red().bold()),
        "1"   => solus_script(),
        "2"   => fedora_script(),
        "3"   => manjaro_script(),
        "5"   =>  clsmm(),
        _e => distros(),
    }
}

// clear main menu
fn clsmm() {
    clear();
    logo();
    mm();
}

/* DISTRO SCRIPTS */

// solus
fn solus_script() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/distros/Solus/solus.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}
// fedora
fn fedora_script() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/distros/Fedora/fedora.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}
// manjaro
fn manjaro_script() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
        sudo pacman -Syu
        clear
        echo -e "\e[0;92mManjaro script starting..."
        sudo pacman -Syu i3 i3blocks i3status i3lock polybar kitty krita fish ack vim bottom neofetch flameshot variety feh rofi discord python-pip steam telegram-desktop gcolor3 lxappearance picom flatpak xdg-desktop-portal-gtk awesome-terminal-fonts noto-fonts-emoji noto-fonts --noconfirm 
        sudo mkdir -p ~/.appz
        sudo mkdir -p ~/.config/i3/
        sudo mkdir -p ~/.local/share/Trash/files
        sudo tar -C $HOME -xzf src/packages/NoiseTorch_x64.tgz
        sudo tar -xf src/packages/sublime_text_build_4113_x64.tar.xz  -C ~/.appz && gtk-update-icon-cache && flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
        sudo cp src/distros/Manjaro/config ~/.config/i3/
        sudo cp -r src/polybar/ ~/.config/  
        sudo cp -r src/kitty/ ~/.config/
        sudo flatpak install flathub com.rafaelmardojai.Blanket -y 
        sudo cp -r src/dots/.bashrc ~/ 
        sudo chsh -s /usr/bin/fish
        chsh -s /usr/bin/fish
        sudo cp -r src/assets/walls/ ~/
        sudo cp -r src/assets/Themes/Solarized-Dark-Blue src/assets/Themes/Solarized-Dark-Cyan /usr/share/themes/ 
        sudo cp -r src/assets/icons/Tela-blue/ src/assets/icons/Tela-blue-dark/ /usr/share/icons/ 
        sudo cp -r src/dots/.fonts ~/
        sudo cp -r src/dots/git_token ~/
        echo -e "\e[0;92mDONE!"
        echo -e "\e[0;91m!\e[0;1;33mYour system will be \e[0;91mREBOOT!"
        sleep 5
        reboot 
        "#
    )
    .unwrap();

    println!("{}", output);
}

/* My OS */

fn myos() {
    clear();
    logo();
    let (_code, output, _error) = run_script::run_script!(
        r#"
        screenfetch
        exit 0
        "#
    )
    .unwrap();
    
    println!("{}", output);
    mm();
}

/* INFO */
fn info() {
    clear();
    logo();
    
    // about info
    version();
    edition();
    author();
    man();

    println!("\n {} {} {} {}", "5)".cyan(), "Back".yellow(), "0)".cyan(), "QUIT".red().bold());

    eprint!("\n Coose an action: ");

    let mut back = String::new();
     match io::stdin().read_line(&mut back) {
        Ok(_) => {},
        Err(_e) => println!("Ошибка программы!")
    }
    

    match back.trim() {
        "0" => println!("{}", "\n EXIT".red().bold()),
        "5"   => main(),
        _e => info(),
    }

}

// info descr
fn version() {
    let ver:&str="1.1.0";
    println!("{} {}", " VERSION: ".yellow().bold(), ver)
}
fn edition() {
    let ed:&str="Rust";
    let _ = println!("{} {}", " EDITION: ".yellow().bold(), ed);
}
fn author() {
    let auth:&str=" https://github.com/Avdushin";
    let _ = println!("{} {}", " AUTHOR: ".yellow().bold(), auth);
}
fn man() {
    let man:&str="https://telegra.ph/PN3APPLE-ENG-INFO-10-23";
    let _ = println!("{} {}", "\n DOCUMENTATION: ".yellow().bold(), man);
}
/* UNINSTALLER */
fn uninstall() {
    clear();
    logouninstall();
    println!("\n\n{} {} {} {} {} {} {} {} {} {}", " 1)".cyan(), "Solus".blue(), "2)".cyan(), "Fedora\n\n".white(), "3)".cyan(), "Manjaro ".green(),      
   "\n\n 5)".cyan() ,"Back".yellow(), "0)".cyan(), "QUIT\n\n".red().bold());

   let mut undistro = String::new();

   eprint!(" Coose to remove: ");
   
   // menu match logic
   match io::stdin().read_line(&mut undistro) {
       Ok(_) => {},
       Err(_e) => println!("Ошибка программы!")
   }

   uninstall_log(&undistro);

}
fn uninstall_log(undistro: &str) {
    match undistro.trim() {
        "0" => println!("{}", "\n EXIT".red().bold()),
        "1"   => solus_uninstall(),
        "2"   => fedora_uninstall(),
        "3"   => manjaro_uninstall(),
        "5"   =>  clsmm(),
        _e => uninstall(),
    }
}
/* unsinstall scripts */
// solus uninstall
fn solus_uninstall() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/distros/Solus/uninst.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}
// fedora uninstall
fn fedora_uninstall() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/distros/Fedora/uninst.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}
// manjaro uninstall
fn manjaro_uninstall() {
    let (_code, output, _error) = run_script::run_script!(
        r#"
         sh src/src/distros/Manjaro/uninst.sh
         "#
    )
    .unwrap();

    println!("{}", output);
}