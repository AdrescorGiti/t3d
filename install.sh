#!/usr/bin/env bash
set -eo pipefail

DEB_URL="https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d_1.0.0-1_amd64.deb"
RPM_URL="https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d-1.0.0-1.x86_64.rpm"
ARCH_URL="https://github.com/AdrescorGiti/t3d/releases/download/T3D/t3d-1.0.0-1-x86_64.pkg.tar.zst"

# Проверка архитектуры (все пакеты собраны под x86_64 / amd64)
ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ] && [ "$ARCH" != "amd64" ]; then
    echo "❌ Ошибка: пакеты собраны только для x86_64, а у тебя: $ARCH" >&2
    exit 1
fi

# Проверка прав root / sudo
SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    if command -v sudo >/dev/null 2>&1; then
        SUDO="sudo"
    else
        echo "❌ Ошибка: для установки требуются права root либо утилита sudo." >&2
        exit 1
    fi
fi

# Определение дистрибутива
PKG_TYPE=""
PKG_URL=""
PKG_FILE=""

if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "$ID $ID_LIKE" in
        *arch*|*cachyos*|*manjaro*|*endeavouros*|*artix*)
            PKG_TYPE="arch"
            PKG_URL="$ARCH_URL"
            PKG_FILE="t3d-1.0.0-1-x86_64.pkg.tar.zst"
            ;;
        *debian*|*ubuntu*|*pop*|*mint*)
            PKG_TYPE="deb"
            PKG_URL="$DEB_URL"
            PKG_FILE="t3d_1.0.0-1_amd64.deb"
            ;;
        *fedora*|*rhel*|*centos*|*rocky*|*almalinux*|*opensuse*)
            PKG_TYPE="rpm"
            PKG_URL="$RPM_URL"
            PKG_FILE="t3d-1.0.0-1.x86_64.rpm"
            ;;
    esac
fi

# Fallback-проверка по наличию пакетных менеджеров в PATH
if [ -z "$PKG_TYPE" ]; then
    if command -v pacman >/dev/null 2>&1; then
        PKG_TYPE="arch"
        PKG_URL="$ARCH_URL"
        PKG_FILE="t3d-1.0.0-1-x86_64.pkg.tar.zst"
    elif command -v apt >/dev/null 2>&1 || command -v apt-get >/dev/null 2>&1; then
        PKG_TYPE="deb"
        PKG_URL="$DEB_URL"
        PKG_FILE="t3d_1.0.0-1_amd64.deb"
    elif command -v dnf >/dev/null 2>&1 || command -v rpm >/dev/null 2>&1; then
        PKG_TYPE="rpm"
        PKG_URL="$RPM_URL"
        PKG_FILE="t3d-1.0.0-1.x86_64.rpm"
    else
        echo "❌ Не удалось определить пакетный менеджер (pacman, apt, dnf)." >&2
        exit 1
    fi
fi

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT
TARGET="$TMP_DIR/$PKG_FILE"

echo "🔍 Определена система: $PKG_TYPE ($ARCH)"
echo "⬇️  Скачивание: $PKG_FILE..."

if command -v curl >/dev/null 2>&1; then
    curl -fSL --progress-bar "$PKG_URL" -o "$TARGET"
elif command -v wget >/dev/null 2>&1; then
    wget -q --show-progress "$PKG_URL" -O "$TARGET"
else
    echo "❌ Ошибка: не найден ни curl, ни wget." >&2
    exit 1
fi

echo "📦 Установка пакета..."
case "$PKG_TYPE" in
    arch)
        $SUDO pacman -U --noconfirm "$TARGET"
        ;;
    deb)
        if command -v apt >/dev/null 2>&1; then
            $SUDO apt install -y "$TARGET"
        else
            $SUDO dpkg -i "$TARGET" || $SUDO apt-get install -f -y
        fi
        ;;
    rpm)
        if command -v dnf >/dev/null 2>&1; then
            $SUDO dnf install -y "$TARGET"
        else
            $SUDO rpm -Uvh --replacepkgs "$TARGET"
        fi
        ;;
esac

echo "✨ Установка завершена! Запуск: t3d"
