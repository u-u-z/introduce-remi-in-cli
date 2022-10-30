mod animations_cli;
use animations_cli::*;

use clearscreen;
use colored::Colorize;

use std::{thread, time};

fn main() {
    clearscreen::clear().expect("failed to clear screen");

    // Loading animations.
    let loading_msgs = vec![
        "[=      ] Loading .  ",
        "[ =     ] Loading .. ",
        "[  =    ] Loading ...",
        "[   =   ] Loading .. ",
        "[    =  ] Loading .  ",
        "[     = ] Loading    ",
        "[      =] Loading .  ",
        "[     = ] Loading .. ",
        "[    =  ] Loading ...",
        "[   =   ] Loading .. ",
        "[  =    ] Loading .  ",
        "[ =     ] Loading    ",
        "[=      ] Loading    ",
    ];
    play_an_animations(loading_msgs, 3, 100);

    play_text_input_animation_base_text_front("# Hi there, nice to meet u here", "$> ");
    thread::sleep(time::Duration::from_secs(1));
    play_text_input_animation_base_text_front("# Welcome to my personal space", "$> ");
    thread::sleep(time::Duration::from_secs(1));
    play_text_input_animation_base_text_front("# Let me introduce myself.", "$> ");
    thread::sleep(time::Duration::from_secs(1));
    play_text_input_animation_base_text_front("whoami", "$> ");
    thread::sleep(time::Duration::from_secs(2));
    // main part
    println!(
        "\n🦇 I'm {} <i@remi.email>",
        format!("Remi IO").truecolor(231, 215, 255)
    );
    println!(
        "{}\n",
        format!("ID: Remi_IO | remi10 ..etc.").truecolor(70, 70, 70)
    );
    thread::sleep(time::Duration::from_secs(2));

    println!(
        "A {}, speed skating athlete, Coach, Referees.\nGraduated from {}.\n",
        format!("fullstack developer").truecolor(255, 215, 0).bold(),
        format!("Harbin Sport University")
            .truecolor(133, 145, 255)
            .bold()
    );
    thread::sleep(time::Duration::from_secs(2));

    play_text_input_animation_base_text_front("# Now I'm try to STARTUP on Web3.", "$> ");
    play_text_input_animation_base_text_front(
        "# Try to make interesting Dapps with new technologies.",
        "$> ",
    );
    play_text_input_animation_base_text_front(
        "# Be like blockchain smart contract (solidity), Rust and Jamstack (Typescript, Next.js, etc.)",
        "$> ",
    );
    play_text_input_animation_base_text_front("# About SNS & contacts:", "$> ");
    thread::sleep(time::Duration::from_secs(1));
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    thread::sleep(time::Duration::from_secs(1));
    play_text_input_animation_base_text_front("./contact", "$> ");
    thread::sleep(time::Duration::from_secs(2));
    println!("");

    println!(
        "📫 Email: {}",
        format!("i@remi.email / i@linux.dog")
            .truecolor(255, 255, 255)
            .bold()
    );
    thread::sleep(time::Duration::from_secs(1));
    println!(
        "🐦 Twitter: {}",
        format!("@Remi_IO").truecolor(0, 172, 237).bold()
    );
    thread::sleep(time::Duration::from_secs(1));
    println!(
        "🐙 Github: {}",
        format!("github.com/u-u-z").truecolor(30, 30, 30).bold()
    );
    thread::sleep(time::Duration::from_secs(1));
    println!(
        "📱 Telegram: {}",
        format!("@Yuyuko").truecolor(100, 200, 255).bold()
    );
    println!("");
    thread::sleep(time::Duration::from_secs(2));
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("# About my hobbies.", "$> ");
    play_text_input_animation_base_text_front("./hobbies", "$> ");
    thread::sleep(time::Duration::from_secs(2));
    println!("");
    println!("🎀 Kigurumi cosplay (ref. Touhou Project - Remilia Scarlet 🦇)");
    thread::sleep(time::Duration::from_secs(1));
    println!("🥤 Americano coffee");
    thread::sleep(time::Duration::from_secs(1));
    println!("🔒 Crypto theme things.");
    thread::sleep(time::Duration::from_secs(1));
    println!("");
    thread::sleep(time::Duration::from_secs(2));
    play_text_input_animation_base_text_front("# You can see more info from my Twitter!", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front("", "$> ");
    play_text_input_animation_base_text_front(
        "# Thank you for watch! I Hope that we can make friends.",
        "$> ",
    );
}
