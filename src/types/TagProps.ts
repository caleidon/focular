export interface TagProps {
  text: string;
  mode: TagMode;
}

export enum TagMode {
  New,
  Suggested,
  Existing,
  None,
}
