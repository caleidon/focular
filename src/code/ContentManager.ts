import type { FocError } from "./../types/FocError";
import type { Preferences } from "./../../src-tauri/bindings/Preferences";
import { AlertColor, allMetadata, allTags } from "./valuesStore";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import type { Metadata } from "../types/Metadata";
import { cacheDir } from "@tauri-apps/api/path";
import { preferences } from "./settingsStore";
import { alerts } from "./valuesStore";
import { get } from "svelte/store";

async function addFiles(filePaths: string[]): Promise<void> {
  await invoke("add_files", { filePaths: filePaths }).catch(
    (error: FocError) => {
      // TODO: make adding files tracked by progress bar
      showAlert(error.description, AlertColor.Error, 5);
      console.error(error);
    }
  );
}

async function getPreferences(): Promise<void> {
  await invoke("get_preferences", {})
    .then((results: Preferences) => {
      preferences.set(results);
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });
}

async function updatePreferences(): Promise<void> {
  await invoke("update_preferences", { preferences: get(preferences) }).catch(
    (error: FocError) => {
      showErrorAlert(error);
    }
  );
}

async function searchContent(query: string): Promise<Metadata[]> {
  let searchResults: Metadata[];

  await invoke("search_content", { query: query })
    .then((result: Metadata[]) => {
      searchResults = result;
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });

  return searchResults;
}

async function getMetadataByHashes(hashes: Array<string>): Promise<Metadata[]> {
  let foundMetadata: Metadata[];

  await invoke("get_metadata_by_hashes", { hashes: hashes })
    .then((result: Metadata[]) => {
      foundMetadata = result;
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });

  return foundMetadata;
}

async function deleteMetadata(metadata: Array<Metadata>): Promise<void> {
  let hashes = metadata.map((metadata) => metadata.hash);

  await invoke("delete_content", { hashes: hashes }).catch(
    (error: FocError) => {
      showErrorAlert(error);
    }
  );
}

async function openInExplorer(path: string): Promise<void> {
  await invoke("open_in_explorer", { path: path }).catch((error: FocError) => {
    showErrorAlert(error);
  });
}

async function getThumbnailPath(hash: string): Promise<string> {
  let thumbnailPath: string;

  await cacheDir()
    .then((dir) => {
      let thumbnailName = hash.toString().replace(/,/g, "");
      const file = dir + "Focular/thumbnails/" + thumbnailName + ".png";
      thumbnailPath = convertFileSrc(file);
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });

  return thumbnailPath;
}

async function getFileSize(filePath: string): Promise<number> {
  let fileSize: number;

  await invoke("get_file_size", { filePath: filePath })
    .then((result: number) => {
      fileSize = result;
    })
    .catch((error: FocError) => {
      showErrorAlert;
      return error;
    });

  return fileSize;
}

async function updateMetadata(metadata: Metadata): Promise<void> {
  metadata.timestamp_modified = Math.trunc(Date.now() / 1000);

  allMetadata.update((current) => {
    current[current.findIndex((m) => m.hash === metadata.hash)] = metadata;
    return current;
  });

  await invoke("update_content", { metadata: metadata }).catch(
    (error: FocError) => {
      showErrorAlert(error);
    }
  );
}

async function getAllTags(): Promise<void> {
  await invoke("get_all_tags", {})
    .then((results: string[]) => {
      allTags.set(results);
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });
}

function showAlert(
  message: string,
  color: AlertColor,
  durationSeconds: number
): void {
  let randomId: string = Math.random().toString();

  alerts.update((currentAlerts) => {
    currentAlerts.push({
      id: randomId,
      message: message,
      color: color,
    });
    return currentAlerts;
  });

  setTimeout(() => {
    alerts.update((currentAlerts) => {
      let index = currentAlerts.findIndex((alert) => alert.id === randomId);
      currentAlerts.splice(index, 1);
      return currentAlerts;
    });
  }, durationSeconds * 1000);
}

function showErrorAlert(error: FocError): void {
  showAlert(
    "Error " + error.code + ": " + error.description,
    AlertColor.Error,
    5
  );
  console.error(error);
}

export {
  addFiles,
  searchContent,
  deleteMetadata as deleteContent,
  openInExplorer,
  getMetadataByHashes,
  getThumbnailPath,
  getFileSize,
  getAllTags,
  updateMetadata,
  getPreferences,
  updatePreferences,
  showAlert,
  showErrorAlert,
};
