<script lang="ts">
  import type { Metadata } from "../../../../types/Metadata";
  import Fuse from "fuse.js";
  import {
    allMetadata,
    allTags,
    selectedMetadata,
  } from "../../../../code/valuesStore";
  import Tag from "./Tag.svelte";
  import { updateMetadata } from "../../../../code/ContentManager";
  import { TagMode, type TagProps } from "../../../../types/TagProps";

  export let pickerMode: boolean;

  let searchText: string = "";
  let availableTags: Array<TagProps> = [];
  let shownTags: Array<TagProps> = [];
  let metadata: Metadata;

  findAvailableTags();

  function searchTags() {
    searchText = searchText.replace(";", "");
    let query = searchText.trim();

    const fuse = new Fuse(availableTags, {
      keys: ["text"],
    });

    shownTags =
      query.length > 0
        ? fuse.search(query).map((result) => result.item)
        : availableTags;

    if (query.length > 0) {
      let pushSuggestion = true;
      availableTags.forEach((availableTag) => {
        if (availableTag.text === query) {
          pushSuggestion = false;
          return;
        }
      });
      if (pushSuggestion) {
        shownTags.unshift({
          text: searchText,
          mode: TagMode.New,
        });
      }
    }
  }

  function findSuggestedTags(metadata: Metadata): Array<TagProps> {
    // TODO: add distinct graphics for suggested, existing from others, existing from ours
    let suggestedTags: Array<TagProps> = [];
    let splitName = metadata.name.split(" ");

    for (const namePart of splitName) {
      if (namePart.length > 2) {
        let fixedNamePart = namePart.toLowerCase().replace(/[^a-zA-Z0-9]/g, "");

        if (
          metadata.tags?.includes(fixedNamePart) ||
          $allTags.includes(fixedNamePart)
        ) {
          continue;
        }

        suggestedTags = [
          ...suggestedTags,
          { text: fixedNamePart, mode: TagMode.Suggested },
        ];
      }
    }

    return suggestedTags;
  }

  function findAvailableTags() {
    metadata =
      $allMetadata[
        $allMetadata.findIndex((m) => m.hash === $selectedMetadata[0].hash)
      ];

    if (pickerMode) {
      let suggestedTags = findSuggestedTags(metadata);
      let existingTags = $allTags.map((tag) => {
        return {
          text: tag,
          mode: shouldHighlightTag(tag) ? TagMode.Existing : TagMode.None,
        };
      });

      availableTags = [...suggestedTags, ...existingTags];
    } else {
      availableTags = metadata.tags.map((tag) => {
        return {
          text: tag,
          mode: shouldHighlightTag(tag) ? TagMode.Existing : TagMode.None,
        };
      });
    }

    availableTags.sort((tag1, tag2) => (tag1.mode === TagMode.None ? -1 : 1));
    availableTags.sort((tag1, tag2) =>
      metadata.tags?.includes(tag1.text) && !metadata.tags?.includes(tag2.text)
        ? 1
        : -1
    );

    shownTags = availableTags;
  }

  function shouldHighlightTag(tag: string): boolean {
    return pickerMode && metadata.tags?.includes(tag);
  }

  function removeTagFromMetadata(tag: string) {
    metadata.tags = metadata.tags.filter((t) => t !== tag);

    if (
      $allTags.includes(tag) &&
      !$allMetadata.some((metadata) => metadata.tags?.includes(tag))
    ) {
      $allTags = $allTags.filter((t) => t !== tag);
    }

    updateAndRefresh();
  }

  function addTagToMetadata(tag: string) {
    metadata.tags = metadata.tags ? [...metadata.tags, tag] : [tag];

    if (!$allTags.includes(tag)) {
      $allTags = [...$allTags, tag];
    }

    searchText = "";
    updateAndRefresh();
  }

  function onTagClicked(tag: string) {
    if (!pickerMode) {
      return;
    }

    if (metadata.tags?.includes(tag)) {
      removeTagFromMetadata(tag);
    } else {
      addTagToMetadata(tag);
    }
  }

  function updateAndRefresh() {
    searchTags();
    findAvailableTags();
    updateMetadata(metadata).then(() => {
      window.dispatchEvent(new CustomEvent("onTagsChanged"));
    });
  }
</script>

<!-- // TODO: treba slozit max width and height i da se scrolla ako ide prek -->
<div class=" bg-gray-500 p-1 rounded-md" class:w-64={pickerMode}>
  {#if pickerMode}
    <input
      bind:value={searchText}
      on:input={searchTags}
      type="text"
      placeholder="Type here"
      class="input w-full max-w-xs"
    />
    <p>Suggested tags:</p>
  {/if}
  <div
    class="flex justify-start items-start flex-wrap bg-gray-800 gap-1 p-2 rounded-lg hover:cursor-pointer max-w-full"
  >
    {#each shownTags as tagProps}
      <Tag
        onClicked={onTagClicked}
        onDeleteButtonPressed={removeTagFromMetadata}
        showDeleteButton={!pickerMode}
        props={tagProps}
      />
    {/each}
  </div>
</div>
