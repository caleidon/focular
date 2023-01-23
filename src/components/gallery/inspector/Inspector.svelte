<script lang="ts">
  import { formatDate, type Metadata } from "../../../types/Metadata";
  import {
    getFileSize,
    getThumbnailPath,
    updateMetadata,
  } from "../../../code/ContentManager";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import autoResize, {
    resizeManually,
  } from "./autoResizingTextarea/autoresize";
  import { afterUpdate, type ComponentProps } from "svelte";
  import { popover } from "../../hooks/popover";
  import TagManager from "./tags/TagManager.svelte";
  import {
    allMetadata,
    metadataToSave,
    selectedMetadata,
    selectedMetadataHashes,
  } from "../../../code/valuesStore";
  import { ContentType } from "../../../types/ContentType";

  const maxFileNameTextareaHeight = 150;
  const maxNotesTextareaHeight = 300;

  let assetUrls: Array<string> = [];

  let fileNameTextArea: HTMLTextAreaElement;
  let notesTextArea: HTMLTextAreaElement;

  let fileName: string;
  let notes: string;
  let fileSizes: number;

  let tagManagerProps: { pickerMode: boolean };

  $: $selectedMetadataHashes && updateInspector();

  afterUpdate(() => {
    if (fileNameTextArea) {
      resizeManually(fileNameTextArea, maxFileNameTextareaHeight);
    }
    if (notesTextArea) {
      resizeManually(notesTextArea, maxNotesTextareaHeight);
    }
  });

  function updateInspector() {
    assetUrls = [];
    fileSizes = 0;

    tagManagerProps = {
      pickerMode: true,
    } as ComponentProps<TagManager>;

    for (const metadata of $selectedMetadata) {
      if (metadata.extension === "gif") {
        assetUrls = [...assetUrls, convertFileSrc(metadata.path)];
      } else {
        getThumbnailPath(metadata.hash).then((thumbnailPath) => {
          assetUrls = [...assetUrls, thumbnailPath];
        });
      }

      if (metadata.content_type !== ContentType.Link) {
        getFileSize(metadata.path).then((fileSizeInBytes) => {
          fileSizes += fileSizeInBytes;
        });
      }
    }

    if ($selectedMetadata.length === 1) {
      fileName = $selectedMetadata[0].name;
      notes = $selectedMetadata[0].notes;
    }
  }

  function formatBytes(bytes: number, decimals = 2): string {
    if (bytes === 0) {
      return "0 Bytes";
    }

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
  }

  function updateMetadataManually() {
    $allMetadata[
      $allMetadata.findIndex((m) => m.hash === $selectedMetadata[0].hash)
    ].name = fileName === "" ? "Untitled" : fileName;

    $allMetadata[
      $allMetadata.findIndex((m) => m.hash === $selectedMetadata[0].hash)
    ].notes = notes === "" ? null : notes;

    if ($metadataToSave.length > 0) {
      updateMetadata($selectedMetadata[0]);
      $metadataToSave = [];
    }
  }

  function onMetadataModified() {
    if (!$metadataToSave.includes($selectedMetadata[0])) {
      $metadataToSave = [...$metadataToSave, $selectedMetadata[0]];
    }
  }
</script>

<div class="flex flex-col overflow-auto p-3">
  {#if $selectedMetadata.length === 0}
    <!-- TODO: big picture in center of inspector saying nothing is selected -->
    <p class="text-center">Try selecting something!</p>
  {/if}

  {#if $selectedMetadata.length === 1}
    <div class="flex justify-center overflow-hidden">
      <img
        class="max-h-[200px] rounded-lg object-contain"
        src={assetUrls[0]}
        alt={$selectedMetadata[0].name}
      />
    </div>

    <textarea
      class="textarea textarea-bordered resize-none mt-2"
      bind:this={fileNameTextArea}
      bind:value={fileName}
      on:input={() => onMetadataModified()}
      on:blur={updateMetadataManually}
      use:autoResize={maxFileNameTextareaHeight}
      placeholder="Enter resource title here"
    />

    <textarea
      class="textarea textarea-bordered resize-none my-2"
      bind:this={notesTextArea}
      bind:value={notes}
      on:input={() => onMetadataModified()}
      on:blur={updateMetadataManually}
      use:autoResize={maxNotesTextareaHeight}
      placeholder="Your beautiful notes live here"
    />

    {#key $selectedMetadataHashes}
      <div
        use:popover={{
          component: TagManager,
          componentProps: tagManagerProps,
          positioning: { placement: "left", xOffset: 0, yOffset: 0 },
          openingEvent: "click",
          closingEvent: "click",
        }}
      >
        {#key $selectedMetadata}
          {#if $selectedMetadata[0]?.tags?.length > 0}
            <TagManager pickerMode={false} />
          {:else}
            <button class="rounded-lg bg-red-300">ADD TEGS</button>
          {/if}
        {/key}
      </div>
    {/key}

    <hr class="mt-2" />

    <p class="my-1 font-bold">Properties</p>
    <!-- This can be done in two ways (with grid and flex): https://play.tailwindcss.com/FjjgIHlrYy?size=300x508 -->
    <div
      class="grid grid-cols-2 children:overflow-hidden children:text-ellipsis children:whitespace-nowrap"
    >
      {#if $selectedMetadata.length === 1}
        <div>Type</div>
        <div>{$selectedMetadata[0].content_type}</div>

        {#if $selectedMetadata[0].content_type === ContentType.Image || $selectedMetadata[0].content_type === ContentType.Video}
          <div>Dimensions</div>
          <div>
            {$selectedMetadata[0].width} x {$selectedMetadata[0].height}
          </div>
        {/if}

        {#if $selectedMetadata[0].content_type === ContentType.Audio}
          <div>Duration</div>
          <div>TODO: implement Duration - requires Rust (and saving in db)</div>
        {/if}

        <div>Date added</div>
        <div>
          {formatDate($selectedMetadata[0].timestamp_created)}
        </div>

        <div>Date updated</div>
        <div>
          {formatDate($selectedMetadata[0].timestamp_modified)}
        </div>
      {/if}
    </div>
  {:else if $selectedMetadata.length > 1}
    <p>{$selectedMetadata.length} items selected</p>
    <!-- MULTIPLE ITEMS SELSECTED TODO

        {#each selectedContentHashes as element}
        <p>{element}</p>
      {/each} -->
  {/if}

  {#if fileSizes > 0}
    <div
      class="grid grid-cols-2 children:overflow-hidden children:text-ellipsis children:whitespace-nowrap"
    >
      <div>Size</div>
      <div>{formatBytes(fileSizes)}</div>
    </div>
  {/if}
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
