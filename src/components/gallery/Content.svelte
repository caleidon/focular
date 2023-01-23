<script lang="ts">
  import type { Metadata } from "../../types/Metadata";
  import { ContextOptionIcon } from "../contextMenu/ContextOptionIcon.svelte";
  import { open } from "@tauri-apps/api/shell";
  import deleteIcon from "@iconify/icons-mdi/delete";
  import { createEventDispatcher, type ComponentProps } from "svelte";
  import { deleteContent, openInExplorer } from "../../code/ContentManager";
  import { ContextOption } from "../contextMenu/ContextOption.svelte";
  import Picture from "./Picture.svelte";
  import Video from "./Video.svelte";
  import {
    selectedMetadataHashes,
    selectedMetadata,
  } from "../../code/valuesStore";
  import ContextMenu from "../contextMenu/ContextMenu.svelte";
  import { popover } from "../hooks/popover";
  import { ContentType } from "../../types/ContentType";
  import Other from "./Other.svelte";

  export let gutter: number;
  export let desiredHeight: number;
  export let metadata: Metadata;

  const dispatch = createEventDispatcher();
  const ComponentMapping = {
    Image: Picture,
    Video: Video,
    Link: Picture,
    Other: Other,
  };

  let contextMenuProps = {
    items: constructContextMenuOptions(),
  } as ComponentProps<ContextMenu>;

  function constructContextMenuOptions(): Array<ContextOption> {
    let contextMenuOptions: Array<ContextOption> = [
      new ContextOption("Open", () => {
        // TODO: this needs to open multiple files and doesn't account for Links yet
        open(metadata.path);
      }),
      new ContextOption("Rename", () => {}),
      new ContextOption("Copy", () => {}),
      new ContextOption("Copy file path", () => {}),
      new ContextOptionIcon(
        "Delete (actually just move to trash)",
        () => {
          deleteContent($selectedMetadata).then(() => {
            dispatch("metadataDeleted");
          });
        },
        deleteIcon,
        "#FF4500"
      ),
    ];

    if (metadata.content_type !== ContentType.Link) {
      contextMenuOptions.splice(
        1,
        0,
        new ContextOption("Open in explorer", () => {
          openInExplorer(metadata.path);
        })
      );
    }

    return contextMenuOptions;
  }

  function onDoubleClickHandler() {
    open(metadata.path);
  }

  function onRightClickHandler() {
    dispatch("selectMetadataByRmb", {
      selectedMetadataHash: metadata.hash,
    });
  }
</script>

<div
  id={metadata.hash}
  class="bg-gray-700 selectable rounded-lg overflow-hidden shadow-lg relative"
  class:selected={$selectedMetadataHashes.includes(metadata.hash)}
  style="width:{(metadata.width * desiredHeight) /
    metadata.height}px; flex-grow:{(metadata.width * desiredHeight) /
    metadata.height}; margin:{gutter}px;"
  use:popover={{
    component: ContextMenu,
    componentProps: contextMenuProps,
    positioning: { placement: "mouse", xOffset: 0, yOffset: 0 },
    openingEvent: "contextmenu",
    closingEvent: "click",
  }}
  on:contextmenu={onRightClickHandler}
  on:dblclick={onDoubleClickHandler}
>
  <svelte:component this={ComponentMapping[metadata.content_type]} {metadata} />
</div>

<style>
  .selected {
    background-color: rgb(255, 153, 0);
  }
</style>
