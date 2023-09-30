import { useParams } from "react-router-dom";

export function ProjectPage() {
  const params = useParams<{ projectId: string }>();
  return <div>You viewing project #{params.projectId ?? "??"}</div>;
}
