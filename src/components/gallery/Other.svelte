<script lang="ts">
  import type { Metadata } from "../../types/Metadata";
  import { preferences } from "../../code/settingsStore";

  export let metadata: Metadata;
</script>

<div>
  <div class="w-32 h-32 bg-teal-600" />
  <p class="text-red-500 text-md">UNSUPPORTED FILE</p>
  <p class="px-1 text-center text-ellipsis overflow-hidden whitespace-nowrap">
    {metadata.name}{$preferences.show_file_extensions
      ? "." + metadata.extension
      : ""}
  </p>
</div>
