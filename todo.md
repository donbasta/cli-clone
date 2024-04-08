## Features

- [x] touch
- [ ] mkdir
- [ ] find
- [ ] grep
- [ ] multithreading for find and grep
- [ ] rm, cp, mv
- [ ] flag

  - [ ] ls -la
  - [ ] check flag yang lain

- [ ] highlight when outputting certain file format
- [ ] auto complete pas tekan tab
  - [ ] pake trie biar cepet dan keren, kalo mager pake iterasi aja juga bisa
  - [ ] harus bisa update command selagi ngetik
    - [ ] tab/autocomplete hanya jalan ketika lagi ngetik argumen kedua, ketiga dll. Ketika sedang mengetik argument pertama (command), autocomplete tidak jalan, atau diganti bukan ke list of content of the directory, tapi ke command-commandnya.

## Other apps

- [ ] create snake game
- [ ] bikin tetris
- [ ] text editor
- [ ] todo list app
  - [ ] pake db local (sqlite?)
  - [ ] bisa query2
  - [ ] basic command: (perlu ganti parsing vars di cmdnya biar tokennya jika digabung dengan kutip jadi gaperlu dipisah by whitespace)
    - `todo add "<activity 1>" "<activity 2>" ... "<activity 3>"`
    - `todo list`
    - `todo done <index>`
    - `todo rm <index>`
    - `todo reset`
- [ ] youtube music player from cli
  - [ ] add to queue
  - [ ] login + authenticate account (?)

## Others

- [ ] testing (+ci/cd and release pipeline)
- [ ] modularity and maintanability
- [ ] colorfulness
