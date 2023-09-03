import classNames from "classnames";
import { ReactNode, useEffect, useRef } from "react";
import s from "./styles.module.css";

type Props = {
  opened?: boolean;
  header?: ReactNode;
  children?: ReactNode;
  onClose?: () => void;
};

export function Modal(props: Props) {
  const { opened, header, children, onClose } = props;
  const ref = useRef<HTMLDivElement | null>(null);
  const onCloseRef = useRef<(() => void) | undefined>(onClose);
  onCloseRef.current = onClose;

  useEffect(() => {
    if (!opened) {
      return;
    }

    function handleClickOutside(event: MouseEvent) {
      const target = event.target as HTMLElement | null;

      if (!document.body.contains(target)) {
        return;
      }

      if (ref.current?.contains(target) || ref.current === target) {
        return;
      }

      onCloseRef.current?.();
    }

    function handleEscape(event: KeyboardEvent) {
      if (event.key === "Escape") {
        onCloseRef.current?.();
      }
    }

    window.addEventListener("mousedown", handleClickOutside);
    window.addEventListener("keydown", handleEscape);

    return () => {
      window.removeEventListener("click", handleClickOutside);
      window.removeEventListener("keydown", handleEscape);
    };
  }, [opened]);

  return (
    <div className={classNames(s.modal, opened && s.modal_opened)}>
      <div className={s.modal__content} ref={ref}>
        <div className={s.modal__header}>{header}</div>
        <div className={s.modal__body}>{children}</div>
      </div>
    </div>
  );
}
