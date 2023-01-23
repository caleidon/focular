<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { Metadata } from "../../types/Metadata";
  import { getThumbnailPath } from "../../code/ContentManager";
  import { preferences } from "../../code/settingsStore";

  export let metadata: Metadata;

  let assetUrl: string;

  if (metadata.extension === "gif") {
    assetUrl = convertFileSrc(metadata.path);
  } else {
    getThumbnailPath(metadata.hash).then((thumbnailPath) => {
      assetUrl = thumbnailPath;
    });
  }
</script>

<div>
  <img
    class="relative w-full pointer-events-none"
    src={assetUrl}
    alt={metadata.name}
  />
  <p class="px-1 text-center text-ellipsis overflow-hidden whitespace-nowrap">
    {metadata.name}{$preferences.show_file_extensions
      ? "." + metadata.extension
      : ""}
  </p>
</div>
