import FolderIcon from "@mui/icons-material/Folder";
import NoteIcon from "@mui/icons-material/Note";
import List from "@mui/material/List";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import ListSubheader from "@mui/material/ListSubheader";
import * as React from "react";
import { Dirs } from "./types";

type DirectoriesProps = {
  dirs: Dirs;
  path: string;
  setPath: React.Dispatch<React.SetStateAction<string>>;
};

export function Directories({ dirs, path, setPath }: DirectoriesProps) {
  return (
    <List
      sx={{ width: "100%", maxWidth: 360, bgcolor: "background.paper" }}
      component="nav"
      aria-labelledby="path"
      subheader={
        <ListSubheader component="div" id="path">
          Current dir: {path}
        </ListSubheader>
      }
    >
      {dirs.map((d) =>
        d.is_dir ? (
          <ListItemButton
            key={d.path}
            onDoubleClick={() => {
              setPath(d.path);
            }}
          >
            <ListItemIcon>
              <FolderIcon />
            </ListItemIcon>
            <ListItemText primary={d.path.replace(path + "/", "")} />
          </ListItemButton>
        ) : (
          <ListItemButton key={d.path}>
            <ListItemIcon>
              <NoteIcon />
            </ListItemIcon>
            <ListItemText primary={d.path.replace(path + "/", "")} />
          </ListItemButton>
        )
      )}
    </List>
  );
}
