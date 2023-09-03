import { useRef, useState } from "react";
import Modal from "../Modal";
import { invoke } from "@tauri-apps/api";

type Props = {
  opened?: boolean;
  onClose?: () => void;
  onSuccess?: (project: any) => void;
};

export function ProjectCreateModal(props: Props) {
  const { opened, onClose, onSuccess } = props;
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(false);
  const [name, setName] = useState("");

  const onSuccessRef = useRef(onSuccess);
  onSuccessRef.current = onSuccess;

  function handleSave(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (isLoading) {
      return;
    }
    setError(false);
    setIsLoading(true);
    invoke<any>("create_project", { name })
      .then((data) => {
        onSuccessRef.current?.(data);
        setIsLoading(false);
      })
      .catch((error) => {
        setError(true);
        console.error(error);
        setIsLoading(false);
      });
  }

  return (
    <Modal header="New project" opened={opened} onClose={onClose}>
      <form onSubmit={handleSave}>
        <div className="field field_row">
          <label className="field__label" htmlFor="new_project_name">
            Name:
          </label>
          <input
            className="field__input"
            id="new_project_name"
            type="text"
            name="name"
            autoComplete="off"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div className="field__error">
          {error && "Ooops! Some error happen. Check console for details"}
        </div>

        <button
          className="button button_primary"
          type="submit"
          disabled={isLoading}
        >
          Save
        </button>
        <button className="button" onClick={onClose} disabled={isLoading}>
          Cancel
        </button>
      </form>
    </Modal>
  );
}
