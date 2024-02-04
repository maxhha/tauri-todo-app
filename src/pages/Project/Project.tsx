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
        position: 1,
        createdAt: "2024-02-04 12:00+03:00",
        updatedAt: "2024-02-04 12:00+03:00",
        isDone: false,
        groupId: 2,
      },
    ],
  };

  return (
    <div className="page page_wide">
      <WindowTitle title={`Project #${params.projectId}`} />
      <h3 className="page__header">{project.name}</h3>
      <div className={s.groups}>
        {project.groups.map((group) => (
          <div className={s.group} key={group.id}>
            <div className={s.group__header}>
              <button
                className={classNames(s.group__open, "button button_small")}
              >
                {group.isOpened ? "^" : "V"}
              </button>
              <div className={s.group__name}>{group.name}</div>
            </div>
            <div
              className={classNames(
                s.group__content,
                !group.isOpened && s.group__content_hidden
              )}
            >
              {"<<content>>"}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
