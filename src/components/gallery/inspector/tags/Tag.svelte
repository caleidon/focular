<script lang="ts">
  import { TagMode, type TagProps } from "../../../../types/TagProps";
  export let showDeleteButton: boolean;
  export let onDeleteButtonPressed: (tag: string) => void;
  export let onClicked: (tag: string) => void;
  export let props: TagProps;

  function onDeleteButtonClicked(
    e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
  ) {
    onDeleteButtonPressed(props.text);
  }
</script>

<div
  class="flex items-start justify-start gap-1 py-1 px-2 rounded-lg bg-slate-600 max-w-full hover:bg-slate-400 hover:cursor-pointer"
  class:bg-orange-500={props.mode === TagMode.Existing}
  class:bg-purple-500={props.mode === TagMode.Suggested}
  on:click={() => onClicked(props.text)}
>
  {#if props.mode === TagMode.New}
    ADD
  {/if}
  <div class="shrink overflow-hidden text-ellipsis text-sm text-white">
    {props.text}
  </div>
  {#if showDeleteButton}
    <!-- TODO: replace delete button with actual icon -->
    <button
      on:click={(e) => {
        onDeleteButtonClicked(e);
      }}
      class="font-mono font-bold leading-5 text-md hover:text-red-600">x</button
    >
  {/if}
</div>
