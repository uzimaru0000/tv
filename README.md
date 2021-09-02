<h1 align="center">:tv: tv</h1>

<p align="center">
<img width="320" src="./.github/images/tv_logo.png">
</p>

<h2 align="center"><em>Format json into table view</em></h2>

[![](https://img.shields.io/github/license/uzimaru0000/tv?style=for-the-badge)](https://github.com/uzimaru0000/tv/blob/master/LICENSE)
[![](https://img.shields.io/github/v/release/uzimaru0000/tv?style=for-the-badge)](https://github.com/uzimaru0000/tv/releases/latest)
![](https://img.shields.io/github/downloads/uzimaru0000/tv/total?style=for-the-badge)

## Usage

```
$ cat test.json
[
  {
    "name": "test",
    "age": 10,
    "lang": "ja"
  },
  {
    "name": "uzimaru",
    "age": 23,
    "lang": "ja"
  },
  {
    "name": "hogehoge",
    "age": 21,
    "lang": "en"
  },
  {
    "name": "hugehuge",
    "age": 32,
    "lang": "en"
  }
]

$ tv test.json
|age|lang|    name|
|---|----|--------|
| 10|  ja|    test|
| 23|  ja| uzimaru|
| 21|  en|hogehoge|
| 32|  en|hugehuge|
```

Run `tv --help` to view detailed usage.

## How to install

### For MacOS

```bash
$ brew install uzimaru0000/tap/tv
```
