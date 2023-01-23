<script lang="ts">
  import Gallery from "../components/gallery/Gallery.svelte";
  import { listen } from "@tauri-apps/api/event";

  let gallery: Gallery;

  listen("web_extension_event", (event) => eventHandler(event)).catch(
    console.error
  );

  function eventHandler(event) {
    switch (event.payload.command) {
      case "saveUrl":
        gallery.refreshGallery();
        break;
    }
  }
</script>

<!-- // TODO: add EmptyState  -->
<Gallery bind:this={gallery} desiredHeight={100} gutter={6} />
