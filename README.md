# Ferry — FTP 服务器

基于 Tauri 2 的跨平台 FTP 服务器应用，可在本机一键启停 FTP 服务，供局域网内设备通过 FTP 客户端访问指定目录。

---

## 功能

- **FTP 服务**：基于 [libunftp](https://github.com/bolcom/libunftp)，支持标准 FTP 协议（含被动模式）。
- **可配置**：端口（默认 2121）、根目录、用户名、密码；根目录在 Android 上默认为内置存储路径。


## 环境要求

- **Node.js** 18+
- **Rust**（stable）与 **Cargo**
- **Tauri CLI**：`npm install` 后通过 `npx tauri` 使用
- **Android 打包**：Android SDK、NDK，以及 `tauri android build` 所需环境（见 [Tauri 文档](https://v2.tauri.app/develop/android/)）

---

## 如何编译

### 安装依赖

```bash
npm install
```

### 开发模式（桌面）

```bash
npm run tauri dev
```

会启动前端开发服务器并打开桌面窗口，修改前端或 Rust 代码会热更新或重新编译。

### 桌面打包

```bash
npm run tauri build
```

产物在 `src-tauri/target/release/bundle/` 下（如 `.exe`、`.dmg`、`.AppImage` 等）。

### Android 打包

```bash
npm run android:build
```

默认会生成通用 APK：  
`src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`。

若已配置签名（在 `src-tauri/gen/android/keystore.properties` 及 `app/build.gradle.kts` 的 `signingConfigs`），则输出为已签名 APK，可直接安装。

#### 生成 AAB（Google Play）

```bash
npm run android:build:aab
```

### 应用图标

使用项目根目录下的正方形源图（如 `app-icon.png`，建议 1024×1024）生成各平台图标：

```bash
npm run icon
```

会更新 `src-tauri/icons/` 下的 PNG、ICO、ICNS 等；桌面与安装包会使用新图标。Android 启动器图标需单独替换 `src-tauri/gen/android/app/src/main/res/mipmap-*/` 下的 `ic_launcher*.png`（详见 `src-tauri/icons/README.md`）。

---

## 项目结构（简要）

```
Ferry/
├── src/                 # 前端源码（Vite + TS）
├── src-tauri/
│   ├── src/             # Rust 后端（FTP、网络、Android 前台服务）
│   ├── icons/           # 应用图标
│   ├── gen/android/     # 生成的 Android 工程
│   └── tauri.conf.json  # Tauri 配置
├── app-icon.png         # 图标源图（用于 npm run icon）
└── package.json
```
