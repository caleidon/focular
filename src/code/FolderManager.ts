import { allFolders } from "./valuesStore";
import type { SmartFolder } from "./../../src-tauri/bindings/SmartFolder";
import { invoke } from "@tauri-apps/api";
import type { FocError } from "../types/FocError";
import { showErrorAlert } from "./ContentManager";

async function validateFolderName(folderName: string): Promise<boolean> {
  let available: boolean;

  await invoke("validate_folder_name", { folderName: folderName })
    .then((result: boolean) => {
      available = result;
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });

  return available;
}

async function validateFolderPath(folderPath: string): Promise<boolean> {
  let available: boolean;

  await invoke("validate_folder_path", { folderPath: folderPath })
    .then((result: boolean) => {
      available = result;
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });

  return available;
}

async function addFolder(
  folderName: string,
  folderPath: string
): Promise<void> {
  await invoke("add_folder", {
    folderName: folderName,
    folderPath: folderPath,
  }).catch((error: FocError) => {
    showErrorAlert(error);
    return error;
  });
}

async function getAllFolders(): Promise<void> {
  await invoke("get_all_folders", {})
    .then((results: Array<SmartFolder>) => {
      allFolders.set(results);
    })
    .catch((error: FocError) => {
      showErrorAlert(error);
      return error;
    });
}

export { validateFolderName, validateFolderPath, addFolder, getAllFolders };
