#!/usr/bin/env python3
"""
Скрипт для автоматического исправления warnings в Rust проекте
"""

import os
import re
import subprocess

def run_cargo_check():
    """Запустить cargo check и получить warnings"""
    try:
        result = subprocess.run(['cargo', 'check'], 
                              capture_output=True, text=True, cwd='.')
        return result.stdout, result.stderr
    except Exception as e:
        print(f"Ошибка запуска cargo check: {e}")
        return "", ""

def fix_unused_imports(file_path, warnings):
    """Исправить неиспользуемые импорты"""
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Найти все warnings о неиспользуемых импортах для этого файла
    file_warnings = [w for w in warnings if file_path in w and 'unused import' in w]
    
    for warning in file_warnings:
        # Извлечь имя неиспользуемого импорта
        match = re.search(r"unused import: `([^`]+)`", warning)
        if match:
            import_name = match.group(1)
            # Закомментировать импорт
            pattern = f"use {re.escape(import_name)};"
            replacement = f"// use {import_name};"
            content = re.sub(pattern, replacement, content)
            
            # Также исправить импорты в скобках
            pattern = f"use ([^;]+{{{re.escape(import_name)}[^}}]*}});"
            replacement = lambda m: f"// use {m.group(1)};"
            content = re.sub(pattern, replacement, content)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

def fix_unused_variables(file_path, warnings):
    """Исправить неиспользуемые переменные"""
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Найти все warnings о неиспользуемых переменных для этого файла
    file_warnings = [w for w in warnings if file_path in w and 'unused variable' in w]
    
    for warning in file_warnings:
        # Извлечь имя неиспользуемой переменной
        match = re.search(r"unused variable: `([^`]+)`", warning)
        if match:
            var_name = match.group(1)
            # Добавить префикс _ к переменной
            pattern = f"\\b{re.escape(var_name)}\\b"
            replacement = f"_{var_name}"
            content = re.sub(pattern, replacement, content)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

def main():
    print("🔧 Исправление warnings в Rust проекте...")
    
    # Получить warnings
    stdout, stderr = run_cargo_check()
    all_output = stdout + stderr
    
    # Разделить на отдельные warnings
    warnings = [line.strip() for line in all_output.split('\n') if 'warning:' in line]
    
    print(f"Найдено {len(warnings)} warnings")
    
    # Группировать warnings по файлам
    files_to_fix = set()
    for warning in warnings:
        if '-->' in warning:
            file_match = re.search(r'--> ([^:]+):', warning)
            if file_match:
                files_to_fix.add(file_match.group(1))
    
    print(f"Файлов для исправления: {len(files_to_fix)}")
    
    # Исправить каждый файл
    for file_path in files_to_fix:
        print(f"Исправление {file_path}...")
        fix_unused_imports(file_path, warnings)
        fix_unused_variables(file_path, warnings)
    
    print("✅ Исправление завершено!")
    
    # Проверить результат
    print("\n🔍 Проверка результата...")
    stdout, stderr = run_cargo_check()
    new_warnings = [line for line in (stdout + stderr).split('\n') if 'warning:' in line]
    print(f"Осталось warnings: {len(new_warnings)}")

if __name__ == "__main__":
    main()
