Force wallpaper in different workspace [hyprpaper](https://github.com/hyprwm/hyprpaper) [Hyprland](https://github.com/hyprwm/Hyprland) version 0.52.2 above
## Step1 Build
```bash
git clone https://github.com/Elephant9748/hyprland-paper.git 
cargo build --release
```
## Step2 set binaries on hyprland.conf
add line ``~/.config/hypr/hyprland.conf`` 
```
...
exec-once = ~/hyprland-paper/target/release/hyprland-paper &
...
```
## Step3 config hyprpaper.conf
> !Here only use ii-vii, workspace 1,2 = ii.png
```
preload = ~/Pictures/wallhaven.cc/ii.png
preload = ~/Pictures/wallhaven.cc/iii.png
preload = ~/Pictures/wallhaven.cc/iv.png
preload = ~/Pictures/wallhaven.cc/v.png
preload = ~/Pictures/wallhaven.cc/vi.png
preload = ~/Pictures/wallhaven.cc/vii.png
 wallpaper { 
         monitor = HDMI-A-1 
         path = /home/rigel/Pictures/wallhaven.cc/ii.png # <- this line change everytime
         fit_mode = cover
 }
 wallpaper { 
         monitor = eDP-1 
         path = ~/Pictures/wallhaven.cc/ii.png # <- this line change everytime
         fit_mode = cover
 } 
splash = false
ipc = true #off safe battery life

```
Change **default wallpaper**: 
```rust
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
```

restart hyprland
