# версия
cargo --version

project_name : "new-project"

# создание нового проекта с именем $project_name
cargo new project_name

# сборка проекта
cargo build

# сборка и запуск проекта
cargo run

# проверка компиляции проекта
cargo check

# сборка релизной(оптимизированной) версии
cargo build --release

# документация по текущим зависимостям
cargo doc --open