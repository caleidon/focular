<script lang="ts">
  // TODO: file dropper gets left behind when you drag and drop but you scroll down
  import { createEventDispatcher } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  const dispatch = createEventDispatcher();

  let numberOfFiles: number;
  let isDropping = false;

  listen<string[]>("tauri://file-drop-hover", (event) => {
    if (event.payload.length <= 0) {
      return;
    }

    numberOfFiles = event.payload.length;
    isDropping = true;
  });

  listen("tauri://file-drop-cancelled", () => {
    isDropping = false;
  });

  listen<string[]>("tauri://file-drop", (event) => {
    if (event.payload.length <= 0) {
      return;
    }

    isDropping = false;
    dispatch("onFilesDropped", {
      filePaths: event.payload,
    });
  });
</script>

<div class={isDropping ? "droppingFile" : "hidden"}>
  <!--  TODO: ADD DROP ICON -->
  <!-- <DropFile width="60" /> -->
  <p class="text-lg">
    Release to add {numberOfFiles}
    {numberOfFiles > 1 ? "files" : "file"}
  </p>
</div>

<style lang="postcss">
  .droppingFile {
    @apply w-screen h-screen absolute top-0 left-0 flex flex-col items-center justify-center bg-gray-300/50 border-green-400 border-4 rounded-md;
  }
</style>
