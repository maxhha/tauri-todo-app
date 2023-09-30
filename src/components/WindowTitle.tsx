import { appWindow } from "@tauri-apps/api/window";
import { useLayoutEffect, useRef } from "react";

export default function WindowTitle({ title }: { title: string }) {
  const originalRef = useRef<null | string>(null);
  const promiseRef = useRef<Promise<void>>(Promise.resolve());

  useLayoutEffect(() => {
    if (originalRef.current !== null) {
      return;
    }

    promiseRef.current = promiseRef.current.then(async () => {
      try {
        console.log("start appWindow.title");
        const title = await appWindow.title();
        console.log("end appWindow.title", title);
        if (originalRef.current === null) {
          originalRef.current = title;
        }
      } catch (error) {
        console.error(
          "WindowTitle receive error on get original title:",
          error
        );
      }
    });

    async function resetTitle() {
      if (!originalRef.current) {
        return;
      }

      try {
        console.log("start resetTitle");
        await appWindow.setTitle(originalRef.current);
        console.log("finish resetTitle");
      } catch (error) {
        console.error("WindowTitle receive error on reset title:", error);
      }
    }

    return () => {
      if (promiseRef.current) {
        promiseRef.current = promiseRef.current.then(resetTitle);
      } else {
        resetTitle();
      }
    };
  }, []);

  useLayoutEffect(() => {
    promiseRef.current = promiseRef.current.then(async () => {
      try {
        console.log("start appWindow.setTitle", title);
        await appWindow.setTitle(title);
        console.log("end appWindow.setTitle", title);
      } catch (error) {
        console.error("WindowTitle receive error on set title:", error);
      }
    });
  }, [title]);

  return null;
}
