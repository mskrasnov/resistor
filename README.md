# Resistor

![](data/screenshot_tui.png)

![](data/screenshot_gui.png)

Консольная программа для вычисления сопротивления резисторов по цветной маркировке. На данный момент (версия 0.1) поддерживается только вычисление сопротивления резисторов с четырьмя полосами. В будущем будет добавлена возможность измерения сопротивления резисторов и с иными маркировками.

## Сборка и использование

- **Зависимости:** `rustc`, `cargo`

### Сборка

```bash
cargo build --release
```

### Использование

```bash
./target/release/resistor_[gui или tui]
```

Также вы можете добавить двоичный файл `resistor` в любую директорию, указанную в переменной окружения `PATH`, например, в `~/.local/bin`:

```bash
cp ./target/release/resistor_* ~/.local/bin/
```

Запуск в таком случае:

```bash
resistor_[gui или tui]
```

## Донат

> Сбербанк: 2202206252335406
>
> Boosty: [linux-for-arm](https://boosty.to/linux-for-arm/donate)

## TODO

- [-] ~Добавление графического интерфейса GTK4 + `libadwaita`~
- [X] Добавление графического интерфейса Iced
- [ ] Создание пакетов для дистрибутивов
  - [ ] ArchLinux
  - [ ] Debian GNU/Linux
  - [ ] Fedora
- [ ] Нормальный логотип
