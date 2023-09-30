import { useParams } from "react-router-dom";
import WindowTitle from "../../components/WindowTitle";

export function ProjectPage() {
  const params = useParams<{ projectId: string }>();
  return (
    <div>
      <WindowTitle title={`Project #${params.projectId}`} />
      <span>You viewing project #{params.projectId ?? "??"}</span>
    </div>
  );
}
