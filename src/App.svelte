<script lang="ts">
  import { Route, Router } from "svelte-navigator";
  import All from "./pages/All.svelte";
  import SideBar from "./components/sidebar/SideBar.svelte";
  import Untagged from "./pages/Untagged.svelte";
  import { onMount } from "svelte";
  import {
    getAllTags,
    getPreferences,
    updateMetadata,
  } from "./code/ContentManager";
  import {
    metadataToSave,
    alerts,
    AlertColor,
    allFolders,
  } from "./code/valuesStore";
  import Preferences from "./pages/Preferences.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { getAllFolders } from "./code/FolderManager";

  let shutDownModalOpen: boolean = false;

  appWindow.listen("tauri://close-requested", async ({}) => {
    shutDownModalOpen = true;
    console.log("shutdown requested");

    while ($metadataToSave.length > 0) {
      let metadata = $metadataToSave.pop();
      await updateMetadata(metadata);
    }

    await appWindow.close();
  });

  onMount(() => {
    getPreferences();
    getAllTags();
    getAllFolders();
  });
</script>

<html data-theme="dark" class="dark">
  <div class="flex flex-row dark:dark-globals w-screen h-screen text-sm">
    <Router>
      <SideBar />

      <Route path="/">
        <All />
      </Route>

      <Route path="untagged">
        <Untagged />
      </Route>

      <Route path="preferences">
        <Preferences />
      </Route>

      {#each $allFolders as folder}
        <Route path={folder.name}>
          <Preferences />
        </Route>
      {/each}
    </Router>

    <div class:modal-open={shutDownModalOpen} class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Saving...</h3>
        <p class="py-4">
          Focular is saving your data and will quit as soon as it is finished.
        </p>
      </div>
    </div>

    {#if $alerts.length > 0}
      <div
        class="flex flex-col w-screen absolute bottom-0 z-[1000] justify-center items-center"
      >
        {#each $alerts as alert}
          // TODO: extract into component and fix so that they all have correct
          icons // TODO: make error closeable on mouseover and make it not block
          mouse otherwise (because it's currently full-width)
          <div
            class="alert shadow-lg mb-6 mx-6 w-fit max-w-full overflow-auto"
            class:alert-success={alert.color === AlertColor.Success}
            class:alert-info={alert.color === AlertColor.Information}
            class:alert-warning={alert.color === AlertColor.Warning}
            class:alert-error={alert.color === AlertColor.Error}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="stroke-current flex-shrink-0 h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
              /></svg
            >
            <span class="block truncate">{alert.message}</span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</html>

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;

  @layer components {
    .dark-globals {
      @apply text-gray-100;
    }
  }

  /*  TODO: make unified scrollbar */
  ::-webkit-scrollbar-track {
    background-color: rgb(74, 74, 74);
  }

  ::-webkit-scrollbar {
    width: 7px;
  }

  ::-webkit-scrollbar-thumb {
    background-color: rgb(255, 191, 0);
  }
</style>
