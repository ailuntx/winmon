# winmon

`winmon` 是一个给 Windows 用的终端监控工具，当前目标很窄，只做这类机器：

- Windows 10/11 x64
- Intel CPU
- NVIDIA 独显

默认直接起终端界面，也支持命令模式：

```powershell
winmon
winmon pipe -s 1 --device-info
winmon debug
```

## 安装

最稳的方式还是下载 release 里的 zip，解压后运行一次 `winmon.exe`。

首次运行后会把稳定副本和运行时写到 `%APPDATA%\winmon`，后面新开的 `cmd` 或 `PowerShell` 可以直接输入：

```powershell
winmon
```

如果 release 对当前账号可访问，也可以直接用 PowerShell 一键安装：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/ailuntz/winmon/releases/latest/download/install.ps1 | iex"
```

## 说明

- 颜色、视图模式、刷新间隔保存在 `%APPDATA%\winmon\config.json`
- CPU 温度依赖 `OpenHardwareMonitor`
- 某些机器上的部分传感器可能需要管理员权限
- 当前发布包使用静态 CRT，不额外依赖 VC++ 运行库

## 许可证

当前仓库按 `GPL-3.0-only` 发布。

原因很简单：发布包里会分发 `OpenHardwareMonitor` 运行时，当前实现还会把它内嵌进 `winmon.exe`。`macmon` 相关参考和说明保留在 `third_party/licenses`。
