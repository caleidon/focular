import { type Writable, writable } from "svelte/store";
import type { Preferences } from "../../src-tauri/bindings/Preferences";

export const galleryZoomLevel: Writable<number> = writable(150);
export const currentPopover: Writable<{
  isOpen: boolean;
  close: () => void;
}> = writable({ isOpen: false, close: () => {} });

export const preferences: Writable<Preferences> = writable(null);
