enum Command{
    Quit,
    Pause,
    Play(String),
    SetVolume(u32),
}

fn process_command(cmd: Command) {
    match cmd {
        Command::Quit => println!("Goodbye"),
        Command::Pause => println!("Music paused"),
        Command::Play(m) => println!("Playing music {m}"),
        Command::SetVolume(v) => println!("Volume set to {v}"),
    };
}

fn main() {
    let cmd_pl = Command::Play(String::from("Test.mp3"));
    let cmdq = Command::Quit;
    let cmd_pa = Command::Pause;
    let cmd_s = Command::SetVolume(13);

    process_command(cmd_pl);
    process_command(cmd_s);
    process_command(cmd_pa);
    process_command(cmdq);
}