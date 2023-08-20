# Todo app

This is simple TODO app written on [Tauri](https://tauri.app) + [React](https://react.dev) + [Typescript](https://www.typescriptlang.org).

## Plan

- [ ] Create projects
- [ ] Show projects
- [ ] Open project
- [ ] Create group
- [ ] Show groups
- [ ] Create item
- [ ] Show items
- [ ] Update items
- [ ] Mark done/undone item
- [ ] Reorder items
- [ ] Move items between groups
- [ ] Update groups
- [ ] Open/close groups
- [ ] Reorder groups
- [ ] Update project
- [ ] Archive project
- [ ] Configure default project groups

## Models

```mermaid

erDiagram
    PROJECT {
        int id PK
        text name
        date created_at 
        date updated_at "denormalization of max updated_at in TodoItem"
        bool is_active
        date archived_at
    }
    PROJECT ||--o{ GROUP : contains
    GROUP {
        int id PK
        text name
        int position
        bool is_opened
        int project_id FK
    }
    GROUP ||--o{ TODO : contains
    TODO {
        int id PK
        text text
        int position
        date created_at
        date updated_at
        bool is_done
        date done_at

        int group_id FK
    }
```