<h1 align="center">:tv: tv</h1>

<p align="center">
<img src="./.github/images/logo.png">
</p>

<h2 align="center"><em>Format json into table view</em></h2>

[![](https://img.shields.io/github/license/uzimaru0000/tv?style=for-the-badge)](https://github.com/uzimaru0000/tv/blob/master/LICENSE)
[![](https://img.shields.io/github/v/release/uzimaru0000/tv?style=for-the-badge)](https://github.com/uzimaru0000/tv/releases/latest)
![](https://img.shields.io/github/downloads/uzimaru0000/tv/total?style=for-the-badge)

## How to install

### For MacOS

```bash
$ brew install uzimaru0000/tap/tv
```

### Use cargo

```bash
$ cargo install tv-cli
```

## How to use

### USAGE
```
tv [FLAGS] [OPTIONS] [PATH]
```

### FLAGS
```
-h, --help          Prints help information
    --no-headers    Specify that the input has no header row
-r, --recursive     Recursive display
-V, --version       Prints version information
```

### OPTIONS
```
-a, --align <left | center | right | none>                  Table alignment [default: none]
-s, --sort <SORT_KEY>                                       Options for sorting by key
    --style <ascii | sharp | rounded | markdown | plane>    Table style [default: ascii]
```

### ARGS
```
<PATH>    json file path
```

### Example

```
$ cat example.json
[
  {
    "id": 1,
    "name": "Leanne Graham",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "address": {
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "city": "Gwenborough",
      "zipcode": "92998-3874",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      }
    },
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets"
    }
  },
  ...
]

$ tv example.json
+--+------------------------+----------------+-------------------------+-------+---------------------+-------------+-------+
|id|name                    |username        |email                    |address|phone                |website      |company|
+--+------------------------+----------------+-------------------------+-------+---------------------+-------------+-------+
|1 |Leanne Graham           |Bret            |Sincere@april.biz        |...    |1-770-736-8031 x56442|hildegard.org|...    |
|2 |Ervin Howell            |Antonette       |Shanna@melissa.tv        |...    |010-692-6593 x09125  |anastasia.net|...    |
|3 |Clementine Bauch        |Samantha        |Nathan@yesenia.net       |...    |1-463-123-4447       |ramiro.info  |...    |
|4 |Patricia Lebsack        |Karianne        |Julianne.OConner@kory.org|...    |493-170-9623 x156    |kale.biz     |...    |
|5 |Chelsey Dietrich        |Kamren          |Lucio_Hettinger@annie.ca |...    |(254)954-1289        |demarco.info |...    |
|6 |Mrs. Dennis Schulist    |Leopoldo_Corkery|Karley_Dach@jasper.info  |...    |1-477-935-8478 x6430 |ola.org      |...    |
|7 |Kurtis Weissnat         |Elwyn.Skiles    |Telly.Hoeger@billy.biz   |...    |210.067.6132         |elvis.io     |...    |
|8 |Nicholas Runolfsdottir V|Maxime_Nienow   |Sherwood@rosamond.me     |...    |586.493.6943 x140    |jacynthe.com |...    |
|9 |Glenna Reichert         |Delphine        |Chaim_McDermott@dana.io  |...    |(775)976-6794 x41206 |conrad.com   |...    |
|10|Clementina DuBuque      |Moriah.Stanton  |Rey.Padberg@karina.biz   |...    |024-648-3804         |ambrose.net  |...    |
+--+------------------------+----------------+-------------------------+-------+---------------------+-------------+-------+

```
