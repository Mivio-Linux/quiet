# <img src="https://gitea.com/miviodev/quiet/raw/branch/master/quiet_logo.png  " width="32"> quiet

![ENG README](./README.md)

Эта программа позволит вам скрыть ненужные логи, но при ошибке выведет ошибки:
```bash
quiet gut clone # вместо git clone
```
вывод: 
```
[ERROR]: No such file or directory (os error 2)
```
Так-же в последней версии я сделал обработку ошибок не только в shell (например не найдена команда), а еще и в командах:
```
quiet git lig # вместо git log
```
вывод:
```
[ERROR]: git: «lig» не является командой git. Смотрите «git --help».
Самые похожие команды:
	log
```
# Сборка
для сборки нам понадобится `cargo`:
```bash
cargo build -r
```
бинарник будет лежать в `./target/release/`
