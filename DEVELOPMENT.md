# 开发

## 环境

开发和打包都按 Windows x64 走，默认工具链是 `x86_64-pc-windows-msvc`。

本地开发常用入口：

```powershell
powershell -ExecutionPolicy Bypass -File .\run.ps1
powershell -ExecutionPolicy Bypass -File .\run.ps1 debug
powershell -ExecutionPolicy Bypass -File .\run.ps1 pipe -s 1 --device-info
```

`run.ps1` 只给开发和构建用，不进最终发布包。

## 打包

release 包由 `scripts/package.ps1` 生成：

```powershell
.\scripts\package.ps1 -Version v0.1.0 -TargetDir target
```

当前包里只放：

- `winmon.exe`
- `README.txt`
- `third_party/licenses/*`

`OHM` 不再作为外部运行时一起放在 zip 里，首次运行时由 `winmon.exe` 自己写到 `%APPDATA%\winmon\third_party\ohm`。

## 自举

程序启动时会先做一轮自举：

- 把当前 `winmon.exe` 同步到 `%APPDATA%\winmon\winmon.exe`
- 把内嵌的 `OpenHardwareMonitor_x64.exe` 写到 `%APPDATA%\winmon\third_party\ohm`
- 把 `%APPDATA%\winmon` 写进用户 `PATH`

安装脚本和发布流程都依赖这条链，所以不要随便绕开。

## 发布

分两条：

1. push 到普通分支，触发 `check` workflow，验证格式、编译、打包
2. 打 `v*` tag，触发 `release` workflow，上传 zip、安装脚本和哈希文件

如果以后要改资产命名或下载地址，优先一起看：

- `.github/workflows/release.yml`
- `install.ps1`
- `scripts/package.ps1`
