<script lang="ts">
  import type { Metadata } from "../../types/Metadata";
  import { getDirectYoutubeUrl } from "../../code/RequestManager";
  import VideoPlayer from "svelte-video-player";

  export let metadata: Metadata;

  let source: string = "";
  let isYoutubeVideo: boolean = false;
  const regex =
    /^(?:http(?:s)?:\/\/)?(?:www\.)?(?:m\.)?(?:youtu\.be\/|(?:(?:youtube-nocookie\.com\/|youtube\.com\/)(?:(?:watch)?\?(?:.*&)?v(?:i)?=|(?:embed|v|vi|user)\/)))([a-zA-Z0-9\-_]*)/;
  if (regex.test(metadata.path)) {
    getDirectYoutubeUrl(metadata.path).then((result) => {
      source = result;
      isYoutubeVideo = true;
    });
  } else {
    source = metadata.path;
  }
</script>

FIXME

<!-- {#if isYoutubeVideo}
  <h1>TITLE - {metadata.name}</h1>
  <VideoPlayer
    poster="https://res.cloudinary.com/dvm02rtnk/image/upload/c_scale,q_auto,w_1024/v1628058523/blender/Spring_-_Blender_Open_Movie_dah072.jpg"
    {source}
  />
{:else}
  <h1>TITLE - {metadata.name}</h1>
  <iframe src="https://www.youtube.com/" frameborder="0" />
{/if} -->
