import { useParams } from "react-router-dom";
import WindowTitle from "../../components/WindowTitle";
import s from "./Project.module.css";
import classNames from "classnames";

export function ProjectPage() {
  const params = useParams<{ projectId: string }>();
  const project = {
    name: "First project",
    groups: [
      {
        id: 1,
        name: "Questions",
        position: 1,
        isOpened: true,
      },
      {
        id: 2,
        name: "Red",
        position: 2,
        isOpened: true,
      },
      {
        id: 3,
        name: "Green",
        position: 3,
        isOpened: false,
      },
      {
        id: 4,
        name: "Brown",
        position: 4,
        isOpened: false,
      },
    ],
    todos: [
      {
        text: "Open projects",
        position: 1,
        createdAt: "2024-02-04 12:00+03:00",
        updatedAt: "2024-02-04 12:00+03:00",
        isDone: true,
        groupId: 2,
      },
      {
        text: "Open single project",
        position: 2,
        createdAt: "2024-02-04 12:00+03:00",
        updatedAt: "2024-02-04 12:00+03:00",
        isDone: false,
        groupId: 2,
      },
      {
        text: "Super duper long todo Super duper long todo Super duper long todo Super duper long todo Super duper long todo Super duper long todo Super duper long todo",
        position: 3,
        createdAt: "2024-02-04 12:00+03:00",
        updatedAt: "2024-02-04 12:00+03:00",
        isDone: false,
        groupId: 2,
      },
    ],
  };

  const todoByGroup = project.todos
    .sort((a, b) => b.position - a.position)
    .reduce<Map<number, any[]>>((store, todo) => {
      if (store.has(todo.groupId)) {
        store.get(todo.groupId)!.push(todo);
      } else {
        store.set(todo.groupId, [todo]);
      }

      return store;
    }, new Map());

  return (
    <div className="page page_wide">
      <WindowTitle title={`Project #${params.projectId}`} />
      <h3 className="page__header">{project.name}</h3>
      <ol className={s.groups}>
        {project.groups.map((group) => (
          <li className={s.group} key={group.id}>
            <div className={s.group__header}>
              <button
                className={classNames(s.group__open, "button button_small")}
              >
                {group.isOpened ? "^" : "V"}
              </button>
              <span className={s.group__name}>{group.name}</span>
            </div>
            <ol
              className={classNames(
                s.group__content,
                !group.isOpened && s.group__content_hidden
              )}
            >
              {todoByGroup.get(group.id)?.map((todo) => (
                <li className={classNames(s.todo, s.group__item)}>
                  <button
                    className={classNames(
                      s.todo__toggle,
                      "button button_small"
                    )}
                  >
                    {todo.isDone ? "x" : <>&nbsp;</>}
                  </button>
                  <div className={s.todo__content}>
                    <div className={s.todo__text}>{todo.text}</div>
                    <div className={s.todo__updatedAt}>{todo.updatedAt}</div>
                  </div>
                </li>
              )) || <div className={s.contentPlaceholder}>no data</div>}
            </ol>
          </li>
        ))}
      </ol>
    </div>
  );
}
