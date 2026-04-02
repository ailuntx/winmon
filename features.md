# winmon Feature Tracking

对齐目标：`macmon v0.7.0`

说明：
- 这里只跟踪上游 `changelog.md` 里和 `winmon` 相关的用户可见功能/修复。
- 允许实现方式不同，但功能效果尽量对齐。
- macOS 专属能力如果不适合 Windows，会直接标成 `❌`。

状态：
- `✅` 已有
- `🟡` 待补
- `❌` 不做

## v0.7.0

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | HTTP server mode with JSON and Prometheus endpoints | ✅ | 已补 `serve`，支持 `/json` 和 `/metrics` |
| Feature | launchd service install/uninstall for HTTP server | ❌ | `launchd` 是 macOS 专属，Windows 需要另一套方案 |
| Feature | `cpu_usage_pct` metric | ✅ | 已补到 `pipe` / `debug` / `serve` 输出 |
| Feature | RAM usage percentage display in the label | ✅ | 已补到 TUI |
| Feature | Exposed as a library crate | ❌ | 当前以 CLI/TUI 为主，暂不拆库 |
| Fix | Discard bogus sensor temperature readings | ✅ | 已补基础脏值过滤与平滑 |
| Fix | M5 related voltage-states / processor count fixes | ❌ | Apple Silicon 专属，不适用于 `winmon` |

## v0.6.1

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | SoC info output in pipe/JSON mode | ✅ | `winmon pipe --device-info` 已有等价能力 |

## v0.6.0

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | Timestamp field in pipe output | ✅ | `pipe` 输出已带 `timestamp` |
| Fix | Temperature smoothing when sensors are unavailable | ✅ | 已补缺值回退和平滑 |

## v0.5.1

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Improvement | Improved CPU average temperature calculation via SMC | ❌ | SMC 是 macOS 路线，Windows 不适用 |

## v0.5.0

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | Interactive refresh interval hotkeys | ✅ | `-` / `+` 已支持 |
| Feature | `--interval` allowed in any argument position | ✅ | 当前 `interval` 是全局参数 |
| Fix | CPU power reporting for Ultra chips | ❌ | Apple Silicon 专属，不适用 |

## v0.4.2

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | RAM power metric | ❌ | 当前没有可靠通用来源，暂不做 |
| Feature | Sample count limit for pipe command | ✅ | `pipe -s` 已支持 |

## v0.4.0

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | Raw metrics output in JSON via pipe | ✅ | 已支持 |
| Improvement | Smooth interpolation of temperature and power values | 🟡 | 当前刷新接近 1s，但展示层还没做类似插值/平滑 |
| Fix | GPU frequency reporting | ✅ | 已有 GPU 频率，但实现基于 Windows/NVIDIA 路线 |

## v0.3.0

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | Switch chart type | ✅ | `v` 已支持 Sparkline/Gauge 切换 |
| Feature | Settings persistence between sessions | ✅ | 颜色、视图、间隔已持久化到 `%APPDATA%\\winmon\\config.json` |

## v0.2.1

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | Total system power display | 🟡 | UI 已有 `SYS`，但数据源还不稳定，经常为 `N/A` |
| Feature | `--no-color` mode | ❌ | 当前未做，优先级低 |

## v0.2.0

| 类别 | 上游项 | 状态 | 备注 |
| --- | --- | --- | --- |
| Feature | CPU/GPU average temperature display | ✅ | 已支持，CPU 温度走 `OpenHardwareMonitorLib.dll` |
| Feature | Ability to change colors | ✅ | `c` 已支持 |
| Feature | Version label in the UI | ✅ | 已支持 |
| Improvement | Better E/P CPU frequency calculation | ✅ | 已有等价能力，但实现基于 Windows 性能计数器和 OHM |

## 当前最值得补的项

1. `cpu_usage_pct`
2. RAM 百分比显示
3. 温度脏值过滤 / 缺值平滑
4. `serve` 模式（`/json` + `/metrics`）
