import type { ContentType } from "./ContentType";
import type { Status } from "./Status";

export interface Metadata {
  name: string;
  path: string;
  extension: string;
  tags: Array<string>;
  notes: string;
  hash: string;
  content_type: ContentType;
  status: Status;
  width: number;
  height: number;
  duration: string;
  timestamp_created: number;
  timestamp_modified: number;
}

function formatDate(timestamp: number): string {
  var dateTime = new Date(timestamp * 1000);
  var date = dateTime.toLocaleDateString();
  var time = dateTime.toLocaleTimeString();
  return date + " " + time;
}

export { formatDate };
