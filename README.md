<p align="center">
    <img src="./assets/icon.png" style="width: 150px;" alt="Seam" />
</p>

<h2 align="center">
  Seam
</h2>

<h2 align="center">
  <a href="https://github.com/Borber/seam">
    <img src="https://img.shields.io/badge/github-Borber/seam-8da0cb.svg?style=for-the-badge&logo=github" alt="Github"/>
  </a>
  <a href="https://github.com/Borber/seam/releases/latest">
    <img src="https://img.shields.io/github/downloads/Borber/seam/total.svg?style=for-the-badge&color=82E0AA&logo=github" alt="Downloads"/>
  </a>
  <img src="https://img.shields.io/github/license/borber/seam?color=%2398cbed&logo=rust&style=for-the-badge" alt="LICENSE"/>
</h2>

> 原 `SBtream` 项目, 经历 python 不成熟的模仿, Java 重构烂尾, 目前使用 rust 进行重构开发

多平台直播源地址获取

# 待办

欢迎各位大佬 PR , 积极响应, 友善沟通, 快速 CR, 给您最好的开源体验

-   [ ] GUI action 添加便携版本, 方便已经安装了 WebView2 的用户使用
-   [ ] 添加日志模块, 以便于用户反馈问题
-   [ ] 链接识别
    -   [ ] 识别平台
    -   [ ] 识别房间号

# GUI

![GUI](assets/gui.png)

## [详情](crates/gui/README.md)

# CLI

```bash
❯ .\seam.exe -l douyu -i 88080
[  {
    "rate": "超清1",
    "url": "http://url1"
  },
  {
    "rate": "超清2",
    "url": "http://url2"
  }
]
```

## [详情](crates/cli/README.md)

# 下载

[Releases · seam](https://github.com/Borber/seam/releases) 下载 `GUI`/`CLI`可执行文件

# 使用

|                 **平台**                  | **代号** |                             **`<RID>` 位置**                             | **弹幕** | **房间号获取** |
| :---------------------------------------: | :------: | :----------------------------------------------------------------------: | :------: | :------------: |
|    [B 站](https://live.bilibili.com/)     |   Bili   |                    `https://live.bilibili.com/<RID>`                     |    ✅    |       ✅       |
|      [斗鱼](https://www.douyu.com/)       |  Douyu   | `https://www.douyu.com/<RID>` 或 `https://www.douyu.com/xx/xx?rid=<RID>` |          |       ✅       |
|     [抖音](https://live.douyin.com/)      |  Douyin  |                     `https://live.douyin.com/<RID>`                      |          |                |
|         [虎牙](https://huya.com/)         |   Huya   |                         `https://huya.com/<RID>`                         |          |                |
|    [快手](https://live.kuaishou.com/)     | Kuaishou |                   `https://live.kuaishou.com/u/<RID>`                    |          |                |
|         [CC](https://cc.163.com/)         |    Cc    |                        `https://cc.163.com/<RID>`                        |          |                |
|     [花椒](https://www.huajiao.com/)      | Huajiao  |                    `https://www.huajiao.com/l/<RID>`                     |          |                |
|      [艺气山](https://www.173.com/)       |   Yqs    |                       `https://www.173.com/<RID>`                        |          |                |
|      [棉花糖](https://www.2cq.com/)       |   Mht    |                       `https://www.2cq.com/<RID>`                        |          |                |
|       [kk](https://www.kktv5.com/)        |    Kk    |                    `https://www.kktv5.com/show/<RID>`                    |          |                |
|      [千帆直播](https://qf.56.com/)       |    Qf    |                        `https://qf.56.com/<RID>`                         |          |       ✅       |
|      [Now 直播](https://now.qq.com/)      |   Now    |            `https://now.qq.com/pcweb/story.html?roomid=<RID>`            |          |                |
|       [映客](https://www.inke.cn/)        |   Inke   |           `https://www.inke.cn/liveroom/index.html?uid=<RID>`            |          |       ✅       |
|     [afreeca](https://afreecatv.com/)     | Afreeca  |                     `https://bj.afreecatv.com/<RID>`                     |          |                |
| [pandalive](https://www.pandalive.co.kr/) |  Panda   |               `https://www.pandalive.co.kr/channel/<RID>`                |          |                |
|     [flex](https://www.flextv.co.kr/)     |   Flex   |                `https://www.flextv.co.kr/channels/<RID>`                 |          |                |
|     [wink](https://www.winktv.co.kr/)     |   Wink   |                 `https://www.winktv.co.kr/channel/<RID>`                 |          |                |

# 设置

`config.toml` 放置在 `seam` 可执行文件所在目录下

```toml
# [rid]: 房间号
# [title]: 标题
# [time]: 时间戳
# [date]: 日期

[file_name]
# 录制文件标题
video = "[rid]-[title]-[date]-[time]"
# danmu文件标题
danmu = "[rid]-[title]-[date]-[time]"


[headers.global]
user-agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.200"
# 各平台
[headers.douyin]
cookie = "xxxx"

[headers.kuaishou]
cookie = "xxxx"

```

## 平台说明

| **平台** | **代号** | **必须** |
| :------: | :------: | :------: |
|   抖音   |  douyin  |  cookie  |
|   快手   |    ks    |  cookie  |

## 解释

-   headers 固定字段, 不能更改
-   \[headers.xxxx\] xxxx 为支持平台的 子命令, 可在 [README](../README.md) 中找到
-   支持所有合法 headers 字段
-   headers.global 全局生效, 但会被平台单独设置覆盖

[额外说明](./doc/配置说明.md)

# 路线

[seam](https://github.com/users/Borber/projects/4/views/1)

# 相关项目

-   [seamui](https://github.com/kirito41dd/seamui) 由 [kirito41dd](https://github.com/kirito41dd) 开发的`seam`图形化界面
-   [SeamPotPlayer](https://github.com/chen310/SeamPotPlayer/) 由[chen310](https://github.com/chen310) 开发, 直接在 PotPlayer 中调用 seam 播放直播

## 贡献者

[![GitHub Contributors](https://contrib.rocks/image?repo=Borber/seam)](https://github.com/Borber/seam/graphs/contributors)

# 感谢

-   [wbt5/real-url](https://github.com/wbt5/real-url/)
-   [banner](https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Chunky&text=SEAM)
-   [手把手教你破解斗鱼 sign 算法](https://zhuanlan.zhihu.com/p/107330805)

## Star History

<a href="https://github.com/Borber/seam/stargazers">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=Borber/seam&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=Borber/seam&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=Borber/seam&type=Date" />
  </picture>
</a>
