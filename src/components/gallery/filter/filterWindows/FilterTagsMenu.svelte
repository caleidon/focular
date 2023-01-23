<script lang="ts" context="module">
  import { tagFilterSettings } from "../../../../code/filterStore";
  import { get } from "svelte/store";
  import type { Metadata } from "../../../../types/Metadata";
  import { allTags } from "../../../../code/valuesStore";

  export function resetTagFilter() {
    tagFilterSettings.set({
      selectedTags: [],
    });
  }

  export function filterMetadataByTags(metadata: Metadata[]) {
    // TODO: if you remove a tag, we should refresh the list of tags here incase it was selected
    // TODO: thoroughly test this function, not sure if it works properly
    let allowedTags = get(tagFilterSettings).selectedTags;
    return metadata.filter((m) =>
      allowedTags.some((t) => m.tags && m.tags.includes(t))
    );
  }

  export function isTagFilterEnabled() {
    return get(tagFilterSettings).selectedTags.length > 0;
  }
</script>

<script lang="ts">
  type SelectedTagsTab = "all" | "selected";
  interface Tag {
    text: string;
    selected: boolean;
  }

  let selectedTagsTab: SelectedTagsTab = "all";
  let availableTags: Array<Tag> = $allTags.map((tag) => {
    return {
      text: tag,
      selected: $tagFilterSettings.selectedTags.includes(tag),
    };
  });

  function toggleTag(tag: Tag) {
    if (get(tagFilterSettings).selectedTags.includes(tag.text)) {
      $tagFilterSettings.selectedTags = $tagFilterSettings.selectedTags.filter(
        (t) => t !== tag.text
      );
    } else {
      $tagFilterSettings.selectedTags = [
        ...$tagFilterSettings.selectedTags,
        tag.text,
      ];
    }
  }
</script>

<div class="bg-gray-700 opacity-70 hover:opacity-100">
  <div class="tabs">
    <a
      on:click={() => (selectedTagsTab = "all")}
      class="tab tab-bordered"
      class:tab-active={selectedTagsTab === "all"}>All</a
    >
    <a
      on:click={() => (selectedTagsTab = "selected")}
      class="tab tab-bordered"
      class:tab-active={selectedTagsTab === "selected"}>Selected</a
    >
  </div>

  {#if selectedTagsTab == "all"}
    <div class="max-h-80 overflow-auto">
      <div class="grid grid-cols-2">
        {#each availableTags as tag}
          <label class="label cursor-pointer">
            <span class="label-text">{tag.text}</span>
            <input
              bind:checked={tag.selected}
              on:change={() => toggleTag(tag)}
              type="checkbox"
              class="checkbox checkbox-primary"
            />
          </label>
        {/each}
      </div>
    </div>
  {/if}
</div>
