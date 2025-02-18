import os
import shutil
import click


def copy_tree_structure(source, destination, exclude_dirs=None, exclude_files=None):
    if exclude_dirs is None:
        exclude_dirs = set()
    if exclude_files is None:
        exclude_files = set()

    for root, dirs, files in os.walk(source):
        rel_path = os.path.relpath(root, source)
        target_path = os.path.join(destination, rel_path)

        if any(excluded in rel_path for excluded in exclude_dirs):
            continue  # Пропустить исключенные директории

        os.makedirs(target_path, exist_ok=True)

        for file in files:
            if file not in exclude_files:
                shutil.copy2(os.path.join(root, file), target_path)

@click.command()
@click.argument("source")
@click.argument("destination")
@click.option("--exclude-dirs", multiple=True, help="Исключаемые папки")
@click.option("--exclude-files", multiple=True, help="Исключаемые файлы")
def main(source, destination, exclude_dirs, exclude_files):
    if not os.path.exists(source):
        click.echo(f"Ошибка: Исходная папка '{source}' не существует.")
        return
    
    os.makedirs(destination, exist_ok=True)
    copy_tree_structure(source, destination, set(exclude_dirs), set(exclude_files))
    click.echo("Копирование завершено!")

if __name__ == "__main__":
    main()
