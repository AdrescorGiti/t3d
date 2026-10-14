# t3d

Быстрый, автономный 3D-движок для терминала на чистом Rust. Поддерживает 24-битный TrueColor, Z-буфер глубины, освещение по Блинну-Фонгу, субпиксельный режим символами Брайля и управление мышью.

---

## Установка

### 1. Автоматический установщик в одну команду (рекомендуется)

Скрипт автоматически распознает дистрибутив (Arch, Debian, Ubuntu, Fedora, RHEL), скачивает подходящий пакет релиза, устанавливает его в систему и удаляет временные файлы:

```bash
curl -fsSL [https://raw.githubusercontent.com/AdrescorGiti/t3d/main/install.sh](https://raw.githubusercontent.com/AdrescorGiti/t3d/main/install.sh) | bash
```

---

### 2. Прямое скачивание и установка пакетов

* **Debian / Ubuntu / Linux Mint (.deb)**[cite: 2, 5]:
  ```bash
  wget [https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d_1.0.0-1_amd64.deb](https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d_1.0.0-1_amd64.deb)
  sudo apt install -y ./t3d_1.0.0-1_amd64.deb
  ```

* **Arch Linux / Manjaro / CachyOS (.pkg.tar.zst)**[cite: 1, 6]:
  ```bash
  wget [https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d-1.0.0-1-x86_64.pkg.tar.zst](https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d-1.0.0-1-x86_64.pkg.tar.zst)
  sudo pacman -U --noconfirm ./t3d-1.0.0-1-x86_64.pkg.tar.zst
  ```

* **Fedora / RHEL / openSUSE (.rpm)**[cite: 3, 4]:
  ```bash
  wget [https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d-1.0.0-1.x86_64.rpm](https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d-1.0.0-1.x86_64.rpm)
  sudo dnf install -y ./t3d-1.0.0-1.x86_64.rpm
  ```

---

### 3. Сборка из исходников (Cargo)

```bash
git clone [https://github.com/AdrescorGiti/t3d.git](https://github.com/AdrescorGiti/t3d.git)
cd t3d
cargo build --release
sudo install -Dm755 target/release/t3d /usr/bin/t3d
```

---

## Управление

### Клавиатура

| Клавиша | Действие |
| :--- | :--- |
| `1` – `9` | Выбор фигуры (Куб, Пирамида, Сфера, Рубин, Тор, Октаэдр, Икосаэдр, Узел, ДНК) |
| `Tab` / `]` | Следующая фигура |
| `[` | Предыдущая фигура |
| `M` | Смена режима рендера (`Hybrid` → `Solid` → `Braille` → `Wire` → `Points`) |
| `C` | Смена цветовой палитры (Ruby, Cyberpunk, Matrix, Amber CRT, Argon Frost) |
| `B` | Звездный фон (`Выкл` → `Дрейф` → `Гиперпрыжок`) |
| `X` | Отсечение невидимых граней (Backface Culling) |
| `L` | Вращение источника света по орбите |
| `P` | Волновой пульс вершин (Wave Pulse) |
| `G` | Эффект ЭЛТ-монитора и глитчи (CRT Scanlines & Glitch) |
| `H` | Показать / скрыть интерфейс (HUD) |
| `Пробел` | Пауза авто-вращения |
| `W` / `S` / `↑` / `↓` | Вращение по оси X (Pitch) |
| `A` / `D` / `←` / `→` | Вращение по оси Y (Yaw) |
| `+` / `-` | Приближение / удаление камеры (Zoom) |
| `R` | Сброс камеры и углов |
| `Q` / `Esc` | Выход |

### Мышь

* **Зажатие ЛКМ + движение:** свободное 3D-вращение фигуры мышью.
* **Колесо мыши:** приближение и удаление камеры.

---

## Режимы рендера

1. **Hybrid:** полигональное освещение с Z-буфером + контрастные контурные ребра.
2. **Solid:** сплошная заливка полигонов с расчетом зеркальных бликов Блинна-Фонга.
3. **Braille:** субпиксельный каркас символами Брайля ($2 \times 4$ точки на знакоместо).
4. **Wire:** классический векторный каркас с затуханием яркости по глубине.
5. **Points:** облако вершин с выходящими векторами нормалей.

---

## Лицензия

Проект распространяется под лицензией [MIT](LICENSE).
