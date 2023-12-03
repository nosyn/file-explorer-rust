export type Dir = {
  group: string;
  is_dir: boolean;
  len: number;
  modified: string;
  nlink: number;
  path: string;
  perms: string;
  user: string;
};

export type Dirs = Dir[];
