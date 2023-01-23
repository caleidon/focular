import { type Writable, writable, derived } from "svelte/store";
import type { SmartFolder } from "../../src-tauri/bindings/SmartFolder";
import type { Metadata } from "../types/Metadata";

export enum AlertColor {
  Success,
  Information,
  Warning,
  Error,
}

export const allTags: Writable<Array<string>> = writable([]);
export const allMetadata: Writable<Array<Metadata>> = writable([]);
export const selectedMetadataHashes: Writable<Array<string>> = writable([]);
export const selectedMetadata = derived(
  [allMetadata, selectedMetadataHashes],
  ([$allMetadata, $selectedMetadataIds]) => {
    return $allMetadata.filter(($metadata) =>
      $selectedMetadataIds.includes($metadata.hash)
    );
  }
);

export const allFolders: Writable<Array<SmartFolder>> = writable([]);

export const metadataToSave: Writable<Array<Metadata>> = writable([]);
export const alerts: Writable<
  Array<{
    id: string;
    message: string;
    color: AlertColor;
  }>
> = writable([]);
