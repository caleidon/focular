<script lang="ts" context="module">
  import { ContentType } from "../../../../types/ContentType";
  import { typeFilterSettings } from "../../../../code/filterStore";
  import { get } from "svelte/store";

  export function resetTypeFilter() {
    typeFilterSettings.set({
      shownTypes: [
        ContentType.Image,
        ContentType.Gif,
        ContentType.Video,
        ContentType.Audio,
        ContentType.Link,
        ContentType.Other,
      ],
    });
  }

  export function filterMetadataByTypes(metadata: Metadata[]) {
    let allowedTypes = get(typeFilterSettings).shownTypes;
    return metadata.filter((m) => {
      return Object.values(allowedTypes).some(
        (v) => v === +ContentType[m.content_type]
      );
    });
  }

  export function isTypeFilterEnabled() {
    return (
      get(typeFilterSettings).shownTypes.length <
      Object.keys(ContentType).length / 2
    );
  }
</script>

<script lang="ts">
  import type { Metadata } from "../../../../types/Metadata";

  let showLinks = $typeFilterSettings.shownTypes.includes(ContentType.Link);
  let showImages = $typeFilterSettings.shownTypes.includes(ContentType.Image);
  let showVideos = $typeFilterSettings.shownTypes.includes(ContentType.Video);
  let showGifs = $typeFilterSettings.shownTypes.includes(ContentType.Gif);
  let showAudio = $typeFilterSettings.shownTypes.includes(ContentType.Audio);
  let showOther = $typeFilterSettings.shownTypes.includes(ContentType.Other);

  function toggleType(type: ContentType) {
    if ($typeFilterSettings.shownTypes.includes(type)) {
      $typeFilterSettings.shownTypes = $typeFilterSettings.shownTypes.filter(
        (t) => t !== type
      );
      $typeFilterSettings = $typeFilterSettings;
    } else {
      $typeFilterSettings.shownTypes = [
        ...$typeFilterSettings.shownTypes,
        type,
      ];
      $typeFilterSettings = $typeFilterSettings;
    }
  }
</script>

<!-- TODO: extract into separate components -->
<div class="bg-gray-700">
  <div class="form-control">
    <label class="label cursor-pointer">
      <span class="label-text">Links</span>
      <input
        bind:checked={showLinks}
        on:change={() => toggleType(ContentType.Link)}
        type="checkbox"
        class="checkbox checkbox-primary"
      />
    </label>
    <label class="label cursor-pointer">
      <span class="label-text">Images</span>
      <input
        bind:checked={showImages}
        on:change={() => toggleType(ContentType.Image)}
        type="checkbox"
        class="checkbox checkbox-primary"
      />
    </label>
    <label class="label cursor-pointer">
      <span class="label-text">Videos</span>
      <input
        bind:checked={showVideos}
        on:change={() => toggleType(ContentType.Video)}
        type="checkbox"
        class="checkbox checkbox-primary"
      />
    </label>
    <label class="label cursor-pointer">
      <span class="label-text">Gifs</span>
      <input
        bind:checked={showGifs}
        on:change={() => toggleType(ContentType.Gif)}
        type="checkbox"
        class="checkbox checkbox-primary"
      />
    </label>
    <label class="label cursor-pointer">
      <span class="label-text">Audio</span>
      <input
        bind:checked={showAudio}
        on:change={() => toggleType(ContentType.Audio)}
        type="checkbox"
        class="checkbox checkbox-primary"
      />
    </label>
    <label class="label cursor-pointer">
      <span class="label-text">Other</span>
      <input
        bind:checked={showOther}
        on:change={() => toggleType(ContentType.Other)}
        type="checkbox"
        class="checkbox checkbox-primary"
      />
    </label>
  </div>
</div>
