import { invoke } from "@tauri-apps/api/tauri";
import { useCallback, useEffect, useRef, useState } from "react";
import formatDistanceToNow from "date-fns/formatDistanceToNow";
import ProjectCreateModal from "../../components/ProjectCreateModal";

type OffsetDateTime = string;

type Project = {
  id: number;
  name: string;
  created_at: OffsetDateTime;
  updated_at: OffsetDateTime;
  is_active: boolean;
  archived_at: OffsetDateTime | null;
};

export function ProjectsPage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const isLoadingRef = useRef(false);

  const handleAddNew = useCallback(() => {
    setShowCreateModal(true);
  }, []);

  const handleCloseAddNew = useCallback(() => {
    setShowCreateModal(false);
  }, []);

  function getAllProjects() {
    if (isLoadingRef.current) {
      return;
    }

    isLoadingRef.current = true;

    invoke<Project[]>("get_all_projects")
      .then((projects) => {
        setProjects(projects);
      })
      .finally(() => (isLoadingRef.current = false));
  }

  const handleCreateProject = useCallback(() => {
    handleCloseAddNew();
    getAllProjects();
  }, []);

  useEffect(() => {
    getAllProjects();
  }, []);

  return (
    <div className="page">
      <h1 className="page__header">Projects</h1>
      <button className="button button_primary" onClick={handleAddNew}>
        +New
      </button>

      {showCreateModal && (
        <ProjectCreateModal
          opened
          onClose={handleCloseAddNew}
          onSuccess={handleCreateProject}
        />
      )}

      <ul className="projects">
        {projects.map((project) => (
          <li className="project" tabIndex={0} key={project.id}>
            <span className="project__name">{project.name}</span>{" "}
            <span className="project__id">#{project.id}</span>{" "}
            <span className="project__last-update">
              -- {formatDistanceToNow(new Date(project.updated_at))}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}
