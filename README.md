<h1 align="center">:tv: tv</h1>

<p align="center">
<img src="./.github/images/logo.png">
</p>

<h2 align="center"><em>Format json into table view</em></h2>

[![](https://img.shields.io/github/license/uzimaru0000/tv?style=for-the-badge)](https://github.com/uzimaru0000/tv/blob/master/LICENSE)
[![](https://img.shields.io/github/v/release/uzimaru0000/tv?style=for-the-badge)](https://github.com/uzimaru0000/tv/releases/latest)
![](https://img.shields.io/github/downloads/uzimaru0000/tv/total?style=for-the-badge)

## Usage

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

$ tv test.json
+-------+-------+-------------------------+--+------------------------+---------------------+----------------+-------------+
|address|company|email                    |id|name                    |phone                |username        |website      |
+-------+-------+-------------------------+--+------------------------+---------------------+----------------+-------------+
|...    |...    |Sincere@april.biz        |1 |Leanne Graham           |1-770-736-8031 x56442|Bret            |hildegard.org|
|...    |...    |Shanna@melissa.tv        |2 |Ervin Howell            |010-692-6593 x09125  |Antonette       |anastasia.net|
|...    |...    |Nathan@yesenia.net       |3 |Clementine Bauch        |1-463-123-4447       |Samantha        |ramiro.info  |
|...    |...    |Julianne.OConner@kory.org|4 |Patricia Lebsack        |493-170-9623 x156    |Karianne        |kale.biz     |
|...    |...    |Lucio_Hettinger@annie.ca |5 |Chelsey Dietrich        |(254)954-1289        |Kamren          |demarco.info |
|...    |...    |Karley_Dach@jasper.info  |6 |Mrs. Dennis Schulist    |1-477-935-8478 x6430 |Leopoldo_Corkery|ola.org      |
|...    |...    |Telly.Hoeger@billy.biz   |7 |Kurtis Weissnat         |210.067.6132         |Elwyn.Skiles    |elvis.io     |
|...    |...    |Sherwood@rosamond.me     |8 |Nicholas Runolfsdottir V|586.493.6943 x140    |Maxime_Nienow   |jacynthe.com |
|...    |...    |Chaim_McDermott@dana.io  |9 |Glenna Reichert         |(775)976-6794 x41206 |Delphine        |conrad.com   |
|...    |...    |Rey.Padberg@karina.biz   |10|Clementina DuBuque      |024-648-3804         |Moriah.Stanton  |ambrose.net  |
+-------+-------+-------------------------+--+------------------------+---------------------+----------------+-------------+

```

Run `tv --help` to view detailed usage.

## How to install

### For MacOS

```bash
$ brew install uzimaru0000/tap/tv
```

### Use cargo

```bash
$ cargo install tv-cli
```
