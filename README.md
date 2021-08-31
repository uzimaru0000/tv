# tv

<img width="256" src="./.github/images/tv_logo.png">

Format json into table display

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

## How to install

### For MacOS

```bash
$ brew install uzimaru0000/tap/tv
```
