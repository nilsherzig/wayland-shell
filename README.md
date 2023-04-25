# Wayland-Shell

## Why Wayland-Shell?

I am a fan of the Gnome Shell, but I also love Hyprland / Sway. As a result, I wanted to create something to replace the Gnome Shell, albeit on a smaller scale. This project is my first attempt at GTK, so the code may not look pretty. However, I plan to rewrite it once I gain more knowledge about GTK. Currently, the code only displays a basic bar indicating the date and battery level.

![current example](example.png)

To run this project, you need to compile https://github.com/wmww/gtk4-layer-shell yourself.

```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
export LD_LIBRARY_PATH=/usr/local/lib
cargo run 
```


# roadmap / todo

- [ ] modularity (only needed if other people want to use this wacky projekt haha)
- [ ] bar modules
    - [ ] current window name
    - [ ] current workspace (hyprland, others(?))
- [ ] notifications
- [ ] programm launcher (.desktop files) 

# ci 

Test locally with: 

```bash
act -P self-hosted=nektos/act-environments-ubuntu:18.04
```