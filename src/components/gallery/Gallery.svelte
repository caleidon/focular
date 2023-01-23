<script lang="ts">
  import {
    addFiles,
    searchContent,
    showAlert,
    updatePreferences,
  } from "../../code/ContentManager";
  import { galleryZoomLevel, preferences } from "../../code/settingsStore";
  import {
    AlertColor,
    allMetadata,
    selectedMetadataHashes,
  } from "../../code/valuesStore";
  import { onMount } from "svelte";
  import HSplitPane from "../splitPane/HSplitPane.svelte";
  import FileDropper from "./../FileDropper.svelte";
  import Selecto from "svelte-selecto";
  import Content from "./Content.svelte";
  import Inspector from "./inspector/Inspector.svelte";
  import Icon from "@iconify/svelte";
  import filterSettings from "@iconify/icons-mdi/filter-settings";
  import orderAlphabeticalAscending from "@iconify/icons-mdi/order-alphabetical-ascending";
  import FilterBar, { disableAllFilters } from "./filter/FilterBar.svelte";
  import { orderingOrFiltersChanged } from "../../code/filterStore";
  import OrderMenu, { orderMetadata } from "./filter/OrderMenu.svelte";
  import { popover } from "../hooks/popover";
  import { isTagFilterEnabled } from "./filter/filterWindows/FilterTagsMenu.svelte";
  import { isTypeFilterEnabled } from "./filter/filterWindows/FilterTypeMenu.svelte";
  import { isDateFilterEnabled } from "./filter/filterWindows/FilterDateMenu.svelte";
  import {
    dateFilterSettings,
    typeFilterSettings,
    tagFilterSettings,
  } from "../../code/filterStore";
  import SearchableDropdown from "../SearchableDropdown.svelte";
  export let desiredHeight: number;
  export let gutter: number;
  // TODO: maybe just export search directly? - depends if we wanna select just imported file
  export function refreshGallery() {
    search();
  }

  let selecto: Selecto<{}>;
  let searchText: string = "";
  let scroller: HTMLDivElement;
  // TODO: seriously... remove searchRefresher
  let searchRefresher: boolean;
  let showFilterBar: boolean = false;
  let filterBar: FilterBar;
  let suggestedSearches: Array<{
    group: string;
    options: Array<{ text: string }>;
  }>;
  let scrollOptions: {
    container: HTMLDivElement;
    throttleTime: number;
    threshold: number;
  };
  let rmbSelectedMetadataHash: string;

  onMount(() => {
    scrollOptions = {
      container: scroller,
      throttleTime: 30,
      threshold: 0,
    };
  });

  orderingOrFiltersChanged.subscribe(() => {
    search();
  });

  preferences.subscribe(() => {
    updateSuggestedSearches();
  });

  async function search() {
    let results = await searchContent(searchText);
    let filteredResults = filterBar.filterMetadata(results);
    let orderedResults = await orderMetadata(filteredResults);
    $allMetadata = orderedResults;
    searchRefresher = !searchRefresher;

    // remove all selected metadata hashes that we didn't find in the new search batch
    $selectedMetadataHashes.forEach((selectedHash) => {
      if (!$allMetadata.find((metadata) => metadata.hash === selectedHash)) {
        $selectedMetadataHashes = $selectedMetadataHashes.filter(
          (hash) => hash !== selectedHash
        );
      }
    });

    return orderedResults.length;
  }

  function onFilesDropped(event: CustomEvent<{ filePaths: string[] }>) {
    addFiles(event.detail.filePaths).then(() => {
      search();
    });
  }

  function addToSelection(hashes: Array<string>) {
    $selectedMetadataHashes = $selectedMetadataHashes.concat(hashes);
  }

  function removeFromSelection(hashes: Array<string>) {
    $selectedMetadataHashes = $selectedMetadataHashes.filter(
      (hash) => !hashes.includes(hash)
    );
  }

  // TODO: remove bloat
  function onSelectMetadataByRmb(
    event: CustomEvent<{ selectedMetadataHash: string }>
  ) {
    rmbSelectedMetadataHash = event.detail.selectedMetadataHash;

    if (!$selectedMetadataHashes.includes(rmbSelectedMetadataHash)) {
      $selectedMetadataHashes = [rmbSelectedMetadataHash];
    }
  }

  function updateRecentSearches(e: KeyboardEvent, force: boolean) {
    // TODO: add some sort of character limit for recommendations
    if (e?.code === "Enter" || e?.code === "NumpadEnter" || force) {
      let historyText = searchText.trim();

      if (historyText.length === 0) {
        return;
      }

      if ($preferences.recent_searches.length < 20) {
        $preferences.recent_searches.push(historyText);
      } else {
        $preferences.recent_searches.shift();
        $preferences.recent_searches.push(historyText);
      }

      updatePreferences();
    }
  }

  function updateSuggestedSearches() {
    if (!$preferences || $preferences?.recent_searches.length === 0) {
      return;
    }

    let allSearches = $preferences.recent_searches;
    let mostCommonSearches = findMostCommonSearches(allSearches.slice());
    let recentSearches = [];

    // remove duplicates
    recentSearches = allSearches.filter(
      (item, index) => allSearches.indexOf(item) === index
    );

    // remove most common searches and grab the last five
    recentSearches = recentSearches
      .filter((search) => {
        return !mostCommonSearches.includes(search);
      })
      .slice(-10)
      .reverse();

    suggestedSearches = [];

    suggestedSearches.push({
      group: "Suggested",
      options: mostCommonSearches.map((search) => {
        return { text: search };
      }),
    });

    suggestedSearches.push({
      group: "Recent",
      options: recentSearches.map((search) => {
        return { text: search };
      }),
    });
  }

  function findMostCommonSearches(arr: Array<string>, count: number = 3) {
    let mostCommonSearches = [];
    for (let i = 0; i < count; i++) {
      let mostCommonSearch = arr
        .sort(
          (a, b) =>
            arr.filter((v) => v === a).length -
            arr.filter((v) => v === b).length
        )
        .pop();
      mostCommonSearches.push(mostCommonSearch);
      arr = arr.filter((v) => v !== mostCommonSearch);
    }

    return mostCommonSearches;
  }
</script>

<div class="flex flex-col w-full h-full overflow-auto">
  <FileDropper on:onFilesDropped={onFilesDropped} />

  <div class="bg-gray-700 sticky z-[1000]">
    <!-- TODO: add button on right (x) for clearing if text is entered -->
    <SearchableDropdown
      bind:inputText={searchText}
      on:input={search}
      on:click={search}
      on:blur={() => {
        updateRecentSearches(null, true);
      }}
      on:keydown={(e) => {
        updateSuggestedSearches();
        updateRecentSearches(e, false);
      }}
      options={suggestedSearches}
      placeholder="Search here!"
    />

    <input
      bind:value={$galleryZoomLevel}
      class="range range-xs w-40"
      max="400"
      min="50"
      step="10"
      type="range"
    />

    <div class="indicator group">
      <span
        class="indicator-item badge badge-secondary invisible group-hover:visible cursor-pointer"
        class:hidden={!isTagFilterEnabled() &&
          $tagFilterSettings &&
          !isTypeFilterEnabled() &&
          $typeFilterSettings &&
          !isDateFilterEnabled() &&
          $dateFilterSettings}
        on:click={disableAllFilters}>X</span
      >
      <button
        on:click={() => (showFilterBar = !showFilterBar)}
        class="btn btn-square"
      >
        <Icon icon={filterSettings} width="30" />
      </button>
    </div>

    <button
      class="btn btn-square"
      use:popover={{
        component: OrderMenu,
        componentProps: {},
        positioning: { placement: "bottom", xOffset: 0, yOffset: 0 },
        openingEvent: "click",
        closingEvent: "click",
      }}
    >
      <Icon icon={orderAlphabeticalAscending} width="30" />
    </button>
  </div>

  <FilterBar bind:this={filterBar} {showFilterBar} {searchText} {search} />

  <Selecto
    bind:this={selecto}
    dragContainer={".selectable-area"}
    hitRate="1"
    {scrollOptions}
    selectByClick={true}
    selectFromInside={true}
    selectableTargets={[".selectable"]}
    continueSelect={false}
    continueSelectWithoutDeselect={true}
    toggleContinueSelect={"shift"}
    toggleContinueSelectWithoutDeselect={"ctrl"}
    ratio={0}
    on:scroll={({ detail: e }) => {
      scroller.scrollBy(e.direction[0] * 15, e.direction[1] * 15);
    }}
    on:selectStart={({ detail: e }) => {
      if (rmbSelectedMetadataHash != null) {
        if (!e.selected.includes(rmbSelectedMetadataHash)) {
          removeFromSelection([rmbSelectedMetadataHash]);
        }

        rmbSelectedMetadataHash = null;
      }
    }}
    on:select={({ detail: e }) => {
      e.removed.forEach((element) => {
        element.classList.remove("selected");
      });
      e.added.forEach((element) => {
        element.classList.add("selected");
      });
    }}
    on:selectEnd={({ detail: e }) => {
      removeFromSelection(e.removed.map((element) => element.id));
      addToSelection(e.selected.map((element) => element.id));
    }}
  />

  <div class="flex flex-grow flex-shrink min-h-0">
    <HSplitPane
      leftPaneSize="75%"
      rightPaneSize="25%"
      minLeftPaneSize="500px"
      minRightPaneSize="250px"
      showSeparator={true}
    >
      <div
        bind:this={scroller}
        on:scroll={() => selecto.checkScroll()}
        class="w-full h-full selectable-area p-2 overflow-auto"
        slot="left"
      >
        <div class="flex flex-wrap after:flex-grow-[1e4] after:min-w-[20%]">
          {#key searchRefresher}
            {#each $allMetadata as metadata}
              <Content
                bind:metadata
                on:metadataDeleted={() => {
                  $selectedMetadataHashes = [];
                  search();
                }}
                on:selectMetadataByRmb={onSelectMetadataByRmb}
                desiredHeight={$galleryZoomLevel}
                {gutter}
              />
            {/each}
          {/key}
        </div>
      </div>

      <div class="w-full h-full overflow-auto" slot="right">
        <Inspector />
      </div>
    </HSplitPane>
  </div>
</div>

<style>
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
