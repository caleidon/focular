<script lang="ts">
  import ContextMenuDividerComponent, {
    ContextMenuDivider,
  } from "./ContextMenuDivider.svelte";
  import ContextOptionComponent, {
    ContextOption,
  } from "./ContextOption.svelte";
  import ContextOptionIconComponent, {
    ContextOptionIcon,
  } from "./ContextOptionIcon.svelte";
  import { currentPopover } from "../../code/settingsStore";

  export let items: Array<ContextOption>;

  const OptionMapping = {
    [ContextOption.name]: ContextOptionComponent,
    [ContextOptionIcon.name]: ContextOptionIconComponent,
    [ContextMenuDivider.name]: ContextMenuDividerComponent,
  };
</script>

<div class="bg-gray-900 rounded-lg p-2">
  {#each items as item}
    <div
      on:click={() => {
        $currentPopover.close();
      }}
    >
      <svelte:component
        this={OptionMapping[item.constructor.name]}
        option={item}
      />
    </div>
  {/each}
</div>
