<script context="module" lang="ts">
  import Icon, { type IconifyIcon } from "@iconify/svelte";
  import { ContextOption } from "./ContextOption.svelte";

  export class ContextOptionIcon extends ContextOption {
    icon: IconifyIcon;
    color: string;

    constructor(
      text: string,
      onClick: () => void,
      icon: IconifyIcon,
      color: string = "default"
    ) {
      super(text, onClick);
      this.icon = icon;
      this.color = color;
    }
  }
</script>

<script lang="ts">
  export let option: ContextOptionIcon;
</script>

<div
  class="flex gap-1 px-2 py-1 text-md text-white cursor-default circle rounded-lg hover:bg-gray-700"
  style="color: {option.color == 'default' ? '' : option.color};"
  on:click={option.onClick}
>
  <Icon class="m-0 p-0" height="25" icon={option.icon} width="25" />
  {option.text}
</div>
<!-- <div
  on:click={option.onClick}
  class="px-2 py-1 text-md flex items-center gap-1 text-white cursor-default circle rounded-lg hover:bg-gray-700 whitespace-nowrap"
>
  {option.text}
</div> -->
