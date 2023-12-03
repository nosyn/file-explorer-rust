import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { COMMAND } from "./Command";
import { Directories } from "./Directories";
import { Dirs } from "./types";
import { Button } from "@mui/material";

function App() {
  const [path, setPath] = useState("");
  const [dirs, setDirs] = useState<Dirs>([]);
  const parentPath =
    path.split("/").length === 2 ? "/" : path.split("/").slice(0, -1).join("/");
  const showHidden = false;

  const findHomePath = async () => {
    const res = await invoke(COMMAND.LIST_HOME, { showHidden }).catch((err) => {
      console.error(err);
    });

    setPath((res as string).trim());
  };

  const findFiles = async (path: string) => {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const res = await invoke(COMMAND.LIST_DIR, { path, showHidden }).catch(
      (err) => {
        console.error(err);
      }
    );

    setDirs(res as Dirs);
  };

  useEffect(() => {
    findHomePath();
  }, []);

  useEffect(() => {
    findFiles(path);
  }, [path]);

  return (
    <div className="container">
      <Directories dirs={dirs} path={path} setPath={setPath} />
      {path !== "/" && (
        <Button
          onDoubleClick={() => {
            setPath(parentPath);
          }}
          variant="text"
        >
          Go back to parent dir: {parentPath}
        </Button>
      )}
    </div>
  );
}

export default App;
