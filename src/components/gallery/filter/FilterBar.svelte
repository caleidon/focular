<script lang="ts" context="module">
  export function disableAllFilters() {
    resetTagFilter();
    resetTypeFilter();
    resetDateFilter();
  }
</script>

<script lang="ts">
  import { onDestroy, onMount, type ComponentProps } from "svelte";
  import { popover } from "../../hooks/popover";
  import {
    isTagFilterEnabled,
    filterMetadataByTags,
    resetTagFilter,
  } from "./filterWindows/FilterTagsMenu.svelte";
  import {
    isTypeFilterEnabled,
    filterMetadataByTypes,
    resetTypeFilter,
  } from "./filterWindows/FilterTypeMenu.svelte";
  import {
    isDateFilterEnabled,
    filterMetadataByDate,
    resetDateFilter,
  } from "./filterWindows/FilterDateMenu.svelte";
  import {
    dateFilterSettings,
    typeFilterSettings,
    tagFilterSettings,
  } from "../../../code/filterStore";
  import FilterDateMenu from "./filterWindows/FilterDateMenu.svelte";
  import FilterTypeMenu from "./filterWindows/FilterTypeMenu.svelte";
  import FilterTagsMenu from "./filterWindows/FilterTagsMenu.svelte";
  import type { Metadata } from "../../../types/Metadata";

  export let showFilterBar: boolean;
  export let searchText: string;
  export let search: () => Promise<number>;

  let typeMenuProps = {} as ComponentProps<FilterTypeMenu>;
  let dateMenuProps = {} as ComponentProps<FilterDateMenu>;
  let tagsMenuProps = {} as ComponentProps<FilterTagsMenu>;

  onMount(() => {
    window.addEventListener("onTagsChanged", searchAfterTagsChanged, false);
  });

  onDestroy(() => {
    window.removeEventListener("onTagsChanged", searchAfterTagsChanged, false);
  });

  function searchAfterTagsChanged() {
    if (isTagFilterEnabled()) {
      search().then((foundMetadata) => {
        if (foundMetadata === 0) {
          resetTagFilter();
        }
      });
    }
  }

  export function filterMetadata(metadata: Array<Metadata>) {
    if (isTypeFilterEnabled()) {
      metadata = filterMetadataByTypes(metadata);
    }

    if (isTagFilterEnabled()) {
      metadata = filterMetadataByTags(metadata);
    }

    if (isDateFilterEnabled()) {
      metadata = filterMetadataByDate(metadata);
    }

    return metadata;
  }
</script>

{#if showFilterBar}
  <div class="bg-slate-500 py-2">
    <div class="indicator group">
      <span
        class="indicator-item badge badge-secondary invisible group-hover:visible cursor-pointer"
        class:hidden={!isTagFilterEnabled() && $tagFilterSettings}
        on:click={resetTagFilter}>X</span
      >
      <button
        use:popover={{
          component: FilterTagsMenu,
          componentProps: tagsMenuProps,
          positioning: { placement: "bottom", xOffset: 0, yOffset: 0 },
          openingEvent: "click",
          closingEvent: "click",
        }}
        class="btn btn-sm"
        class:bg-green-500={isTagFilterEnabled() && $tagFilterSettings}
        >Tags</button
      >
    </div>

    <div class="indicator group">
      <span
        class="indicator-item badge badge-secondary invisible group-hover:visible cursor-pointer"
        class:hidden={!isTypeFilterEnabled() && $typeFilterSettings}
        on:click={resetTypeFilter}>X</span
      >
      <button
        use:popover={{
          component: FilterTypeMenu,
          componentProps: typeMenuProps,
          positioning: { placement: "bottom", xOffset: 0, yOffset: 0 },
          openingEvent: "click",
          closingEvent: "click",
        }}
        class="btn btn-sm"
        class:bg-green-500={isTypeFilterEnabled() && $typeFilterSettings}
        >Type</button
      >
    </div>

    <div class="indicator group">
      <span
        class="indicator-item badge badge-secondary invisible group-hover:visible cursor-pointer"
        class:hidden={!isDateFilterEnabled() && $dateFilterSettings}
        on:click={resetDateFilter}>X</span
      >
      <button
        use:popover={{
          component: FilterDateMenu,
          componentProps: dateMenuProps,
          positioning: { placement: "bottom", xOffset: 0, yOffset: 0 },
          openingEvent: "click",
          closingEvent: "click",
        }}
        class="btn btn-sm"
        class:bg-green-500={isDateFilterEnabled() && $dateFilterSettings}
        >Date</button
      >
    </div>
  </div>
{/if}
