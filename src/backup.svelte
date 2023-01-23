<script lang="ts">
  export let name: string = "Worldss";
  import { YouTube } from "svelte-yt";

  import { listen } from "@tauri-apps/api/event";
  import { copyFile, createDir, BaseDirectory } from "@tauri-apps/api/fs";

  listen("tauri://file-drop-hover", (event) => {
    name = "dragging";
  });

  listen("tauri://file-drop", (event) => {
    name = "dropped";
    console.log(event.payload);
    createDir("test", { dir: BaseDirectory.Home });
    copyFile(event.payload[0], "test/wat.png", { dir: BaseDirectory.Home });
  });

  let videoId = "M7lc1UVf-VE";
  // Can be used to control full YouTube player
  // See https://developers.google.com/youtube/iframe_api_reference#Functions
  let player;

  // See https://developers.google.com/youtube/player_parameters#Parameters
  const options = {
    playerVars: {
      modestbranding: 1,
    },
  };
</script>

<main>
  <h1>Hello {name}!</h1>

  <YouTube bind:player {videoId} {options} />
  <button on:click={() => player.playVideo()}>Play</button>
  <button on:click={() => player.pauseVideo()}>Pause</button>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
