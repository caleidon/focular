<script lang="ts">
  import type { Metadata } from "../../types/Metadata";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import "video.js/dist/video-js.css";
  import "@videojs/themes/dist/sea/index.css";
  import videojs, {
    type VideoJsPlayer,
    type VideoJsPlayerOptions,
  } from "video.js";
  import { onMount } from "svelte";

  export let metadata: Metadata;

  let options: VideoJsPlayerOptions = {
    fluid: true,
  };
  let el: Element;
  let player: VideoJsPlayer;

  onMount(() => {
    player = videojs(el, options, function onPlayerReady() {
      this.on("loadedmetadata", function () {
        // TODO: do something when video loads, probably set the thumbnail here
        // TODO: preload and set thumbnail, but start from beginning when played ???
      });
    });
  });

  let assetUrl: string = convertFileSrc(metadata.path);
  // TODO: in source, the type must support the 3 ones
  // TODO: also use this for audio playback
  // TODO: theme the video
</script>

<div>
  <video-js bind:this={el} controls preload="metadata" class="vjs-theme-sea">
    <!-- <source src="asset:///home/caleidon/Desktop/small.webm" /> -->
    <!-- <source src="http://techslides.com/demos/sample-videos/small.mp4" /> -->
    <source src={assetUrl} />
  </video-js>

  <p class="text-center text-ellipsis overflow-hidden whitespace-nowrap">
    {metadata.name}
  </p>
</div>
