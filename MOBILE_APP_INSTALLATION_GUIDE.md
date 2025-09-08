# Инструкция по установке мобильного приложения на Google Pixel 7

## 📱 Подготовка к установке

### 1. Включение режима разработчика на Google Pixel 7

1. **Откройте настройки** на вашем Google Pixel 7
2. **Перейдите в "О телефоне"** (About phone)
3. **Найдите "Номер сборки"** (Build number)
4. **Нажмите на "Номер сборки" 7 раз подряд**
5. **Введите PIN-код** когда появится запрос
6. **Появится сообщение** "Вы стали разработчиком!"

### 2. Включение отладки по USB

1. **Вернитесь в главное меню настроек**
2. **Перейдите в "Система" → "Параметры разработчика"**
3. **Включите "Отладка по USB"** (USB debugging)
4. **Подтвердите** в появившемся диалоге

### 3. Установка Android Studio и SDK

#### Вариант A: Полная установка Android Studio
1. **Скачайте Android Studio** с официального сайта: https://developer.android.com/studio
2. **Установите Android Studio** с настройками по умолчанию
3. **Откройте Android Studio** и пройдите мастер настройки
4. **Установите Android SDK** через SDK Manager

#### Вариант B: Только Android SDK (рекомендуется)
1. **Скачайте Android SDK Command Line Tools**:
   - Перейдите на https://developer.android.com/studio#command-tools
   - Скачайте "Command line tools only"
2. **Распакуйте архив** в папку `C:\Android\sdk\`
3. **Добавьте в PATH**:
   - `C:\Android\sdk\cmdline-tools\latest\bin`
   - `C:\Android\sdk\platform-tools`

### 4. Установка Node.js и React Native CLI

```bash
# Установите Node.js (версия 18 или выше)
# Скачайте с https://nodejs.org/

# Установите React Native CLI глобально
npm install -g @react-native-community/cli

# Установите Java Development Kit (JDK 17)
# Скачайте с https://adoptium.net/
```

## 🔧 Настройка проекта

### 1. Переход в папку мобильного приложения

```bash
cd blockchain_project/mobile_app
```

### 2. Установка зависимостей

```bash
npm install
```

### 3. Настройка Android SDK

```bash
# Установите переменные окружения
set ANDROID_HOME=C:\Android\sdk
set ANDROID_SDK_ROOT=C:\Android\sdk

# Добавьте в PATH
set PATH=%PATH%;%ANDROID_HOME%\platform-tools;%ANDROID_HOME%\cmdline-tools\latest\bin
```

## 📲 Установка приложения на Google Pixel 7

### 1. Подключение телефона

1. **Подключите Google Pixel 7** к компьютеру через USB-кабель
2. **Разрешите отладку по USB** на телефоне (появится уведомление)
3. **Проверьте подключение**:
   ```bash
   adb devices
   ```
   Должно показать ваш телефон в списке

### 2. Сборка и установка приложения

#### Вариант A: Через React Native CLI (рекомендуется)
```bash
# В папке mobile_app
npx react-native run-android
```

#### Вариант B: Ручная сборка
```bash
# Сборка APK
cd android
./gradlew assembleDebug

# Установка APK
adb install app/build/outputs/apk/debug/app-debug.apk
```

### 3. Альтернативный способ - через Android Studio

1. **Откройте Android Studio**
2. **Откройте проект**: `blockchain_project/mobile_app/android/`
3. **Подключите телефон** и выберите его в списке устройств
4. **Нажмите кнопку "Run"** (зеленая стрелка)

## 🚀 Запуск приложения

### 1. На телефоне
1. **Найдите иконку приложения** "FoodTruck Network" на главном экране
2. **Нажмите на иконку** для запуска
3. **Разрешите необходимые разрешения** (камера, интернет, уведомления)

### 2. Отладка (опционально)
```bash
# Просмотр логов в реальном времени
adb logcat | findstr "ReactNativeJS"

# Перезапуск приложения
adb shell am force-stop com.foodtrucknetwork
adb shell am start -n com.foodtrucknetwork/.MainActivity
```

## 🔧 Устранение неполадок

### Проблема: "adb: command not found"
**Решение**: Убедитесь, что Android SDK установлен и добавлен в PATH

### Проблема: "Device not found"
**Решение**: 
1. Проверьте USB-кабель
2. Включите отладку по USB
3. Разрешите отладку на телефоне
4. Попробуйте другой USB-порт

### Проблема: "Build failed"
**Решение**:
```bash
# Очистите кэш
cd android
./gradlew clean

# Переустановите зависимости
cd ..
npm install
```

### Проблема: "Metro bundler not found"
**Решение**:
```bash
# Запустите Metro bundler отдельно
npx react-native start
```

## 📋 Проверка установки

После успешной установки вы должны увидеть:

1. **Иконку приложения** "FoodTruck Network" на главном экране
2. **Экран приветствия** при первом запуске
3. **Навигационное меню** с опциями для разных ролей пользователей
4. **Возможность подключения** к блокчейн API

## 🔄 Обновление приложения

Для обновления приложения:

```bash
# В папке mobile_app
git pull origin main
npm install
npx react-native run-android
```

## 📞 Поддержка

Если возникли проблемы:

1. **Проверьте логи**: `adb logcat | findstr "ReactNativeJS"`
2. **Перезапустите Metro bundler**: `npx react-native start --reset-cache`
3. **Очистите кэш**: `npx react-native start --reset-cache`
4. **Переустановите приложение**: удалите с телефона и установите заново

---

## 📱 Краткая инструкция для быстрой установки

1. **Включите режим разработчика** (7 нажатий на "Номер сборки")
2. **Включите отладку по USB** в настройках разработчика
3. **Установите Android SDK** и добавьте в PATH
4. **Подключите телефон** и разрешите отладку
5. **Выполните команды**:
   ```bash
   cd blockchain_project/mobile_app
   npm install
   npx react-native run-android
   ```

Приложение будет установлено и запущено на вашем Google Pixel 7!

---
*Инструкция создана для Google Pixel 7 и Windows 10/11*
*Дата создания: $(date)*
