import { RouteObject } from "react-router-dom";
import Projects from "./pages/Projects";

export const routes: RouteObject[] = [
  {
    path: "/",
    index: true,
    element: <Projects />,
  },
];
