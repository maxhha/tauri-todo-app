import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  return (
    <div className="page">
      <h1 className="page__header">Projects</h1>

      <button className="button">+New</button>
      <ul className="projects">
        {["Project 1", "Second Project", "My active project"].map((name, i) => (
          <li className="project" tabIndex={0}>
            <span className="project__name">{name}</span>{' '}
            <span className="project__id">#{i}</span>{' '}
            <span className="project__last-update">-- 3 minutes ago</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
