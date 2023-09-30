import { RouteObject } from "react-router-dom";
import Projects from "./pages/Projects";
import Project from "./pages/Project";

export const routes: RouteObject[] = [
  {
    path: "/",
    index: true,
    element: <Projects />,
  },
  {
    path: "/:projectId",
    element: <Project />,
  },
];
