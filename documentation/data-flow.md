```mermaid
graph LR
    Generator -->|Генерирует| Transfer
    Transfer --> Storage
    Storage --> Pipeline
    Pipeline -->|Рассчитывает| UserStats
    UserStats --> Storage

    subgraph Main
    Generator --> Storage
    Pipeline --> Storage

    end
```