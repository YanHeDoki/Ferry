# Ferry 应用图标

Tauri 打包使用的图标（`tauri.conf.json` 中 `bundle.icon`）：

- `32x32.png`、`128x128.png`、`128x128@2x.png` — 桌面与 Android 生成用
- `icon.ico` — Windows
- `icon.icns` — macOS

当前已用 `ferry.iconset/` 中的图标覆盖了上述 PNG 和 `icon.ico`。

## 从 ferry.iconset 更新图标

若修改了 `ferry.iconset/` 里的图，可同步到 bundle 用：

```bash
cd src-tauri/icons
cp ferry.iconset/icon_32x32.png 32x32.png
cp ferry.iconset/icon_128x128.png 128x128.png
cp ferry.iconset/icon_128x128@2x.png 128x128@2x.png
magick ferry.iconset/icon_16x16.png ferry.iconset/icon_32x32.png ferry.iconset/icon_64x64.png ferry.iconset/icon_128x128.png ferry.iconset/icon_256x256.png ferry.iconset/icon_512x512.png icon.ico
# macOS 图标（需本机执行）：iconutil -c icns ferry.iconset -o icon.icns
```

## 从 icon.svg 重新导出

- **ImageMagick**：`magick -background none -resize 128x128 icon.svg 128x128.png`（同理可导 32、256 等）
- **在线**：[CloudConvert SVG to PNG](https://cloudconvert.com/svg-to-png)
- **macOS 预览**：打开 `icon.svg` → 导出为 PNG，按尺寸重命名
