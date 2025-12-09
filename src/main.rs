use anyhow::{Context, Result};
use hyprland::{data::Workspace, dispatch, event_listener::EventListener, shared::HyprDataActive};
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

fn hyprpaper_conf_path() -> PathBuf {
    let hyprpaper_config = "~/.config/hypr/hyprpaper.conf";
    let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
    let mut path = PathBuf::from(home_dir);
    if hyprpaper_config.starts_with("~") {
        path.push(&hyprpaper_config[2..]);
    }
    path
}

// read hyprpper.conf
#[allow(dead_code)]
fn readconfig_hyprpaper() -> io::Result<String> {
    let path = hyprpaper_conf_path();
    let config_file = File::open(path).expect(">> hyprpaper config file not found");
    let mut buf_reader = BufReader::new(config_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap_or_default();
    print!("READ |");
    println!("{:#?}", &contents.split("\n").collect::<Vec<_>>()[7]);

    Ok(contents)
}

// write hyprpaper.conf
#[allow(dead_code)]
fn writeconfig_hyprpaper(wallpaper: &str) -> Result<()> {
    let path = hyprpaper_conf_path();

    if let Ok(r) = readconfig_hyprpaper() {
        let mut buf = r.split("\n").collect::<Vec<_>>();

        //set wallpaper then write as config file
        //buf[8] todo()
        let _ = std::mem::replace(&mut buf[7], wallpaper);

        let buffers = buf.join("\n");
        let w_paper = File::create(path).expect(">> hyprpaper config file not found");
        let mut buf_writer = BufWriter::new(w_paper);
        let _ = buf_writer.write_all(buffers.as_bytes());
        let _ = buf_writer.flush();
        print!("WRITE |");
        println!("{:#?}", &buf[7]);
    }

    Ok(())
}

// restart hyprpaper in hyprctl
fn restart_hyprpaper() {
    let _ = Command::new("killall")
        .args(["hyprpaper"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Thread failed No bash found.")
        .try_wait()
        .expect("--> Failed to wait clear_clipboard()");

    let _ = dispatch!(Exec, "hyprpaper");
}

fn hyprland_paper_stats() -> Result<()> {
    // default workspace png
    let w_wallpaper = BTreeMap::from([
        (1, "~/Pictures/wallhaven.cc/ii.png"),
        (2, "~/Pictures/wallhaven.cc/ii.png"),
        (3, "~/Pictures/wallhaven.cc/iii.png"),
        (4, "~/Pictures/wallhaven.cc/iv.png"),
        (5, "~/Pictures/wallhaven.cc/v.png"),
        (6, "~/Pictures/wallhaven.cc/vi.png"),
        (7, "~/Pictures/wallhaven.cc/vii.png"),
    ]);
    //change wallpaper
    let w_active = Workspace::get_active().expect(">> cant get active workspace");
    for (key, value) in w_wallpaper {
        if key == w_active.id {
            println!("{key} | {value}");
            let change_wallpaper = "wallpaper = HDMI-A-1, ".to_owned() + value;
            let _ = writeconfig_hyprpaper(&change_wallpaper);
            restart_hyprpaper();
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for _ in rx {
            if let Err(e) = hyprland_paper_stats() {
                eprintln!("{:?}", e);
                return;
            }
        }
    });
    let _ = tx.send(());
    let mut event = EventListener::new();
    let txx = tx.clone();
    event.add_workspace_changed_handler(move |_| {
        let _ = txx.send(());
    });

    drop(tx);

    event.start_listener().context("Start event listener")
}
