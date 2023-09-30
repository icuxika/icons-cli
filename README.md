从png生成跨平台图标集
==========

从[Tauri](https://github.com/tauri-apps/tauri/blob/dev/tooling/cli/src/icon.rs)拷贝出来的图标集生成工具，如果你在使用`Tauri`搭建跨平台桌面应用，也可以使用`yarn tauri icon -o ~/Downloads/icon ~/Documents/Logo/logo1024.png`来达到同样的目的，可以查看文档[icons](https://tauri.app/zh-cn/v1/guides/features/icons)

TODO
----------
- 支持从`svg`直接生成相关文件

开发
----------
```shell
cargo run -- --help
```

构建
----------
```shell
cargo build --release
target/release/icons-cli --help
```

使用
----------
```shell
target/release/icons-cli -i /Users/icuxika/Documents/Logo/logo1240.png -o /Users/icuxika/Downloads/icon
```
构建日志
```shell
[2023-09-30T14:34:12Z INFO  icons_cli] input: "/Users/icuxika/Documents/Logo/logo1240.png"
[2023-09-30T14:34:12Z INFO  icons_cli] output: "/Users/icuxika/Downloads/icon"
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating StoreLogo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square30x30Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square44x44Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square71x71Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square89x89Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square107x107Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square142x142Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square150x150Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square284x284Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating Square310x310Logo.png
[2023-09-30T14:34:12Z INFO  icons_cli::icon] Creating icon.icns
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating icon.ico
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating 32x32.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating 128x128.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating 128x128@2x.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating icon.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-hdpi/ic_launcher_foreground.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-hdpi/ic_launcher_round.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-hdpi/ic_launcher.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-mdpi/ic_launcher_foreground.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-mdpi/ic_launcher_round.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-mdpi/ic_launcher.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xhdpi/ic_launcher_foreground.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xhdpi/ic_launcher_round.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xhdpi/ic_launcher.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xxhdpi/ic_launcher_foreground.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xxhdpi/ic_launcher_round.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xxhdpi/ic_launcher.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xxxhdpi/ic_launcher_foreground.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xxxhdpi/ic_launcher_round.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating mipmap-xxxhdpi/ic_launcher.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating AppIcon-20x20@2x-1.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating AppIcon-20x20@1x.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating AppIcon-20x20@2x.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating AppIcon-20x20@3x.png
[2023-09-30T14:34:13Z INFO  icons_cli::icon] Creating AppIcon-29x29@2x-1.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-29x29@1x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-29x29@2x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-29x29@3x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-40x40@2x-1.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-40x40@1x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-40x40@2x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-40x40@3x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-60x60@2x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-60x60@3x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-76x76@1x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-76x76@2x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-83.5x83.5@2x.png
[2023-09-30T14:34:14Z INFO  icons_cli::icon] Creating AppIcon-512@2x.png
```