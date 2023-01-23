<script lang="ts">
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  // TODO: figure why this type doesn't work (should be IconifyIcon)
  export let icon: IconifyIcon;
  export let tooltipText: string;
</script>

<div class="sidebar-icon group">
  <Icon {icon} width="35" height="35" />

  <span class="sidebar-tooltip group-hover:scale-100">
    {tooltipText}
  </span>
</div>

<style lang="postcss">
  .sidebar-icon {
    @apply relative flex items-center justify-center
    h-12 w-12 my-2 mx-auto shadow-lg bg-gray-800
  text-green-500 hover:bg-green-600 hover:text-white
    rounded-3xl hover:rounded-xl transition-all duration-200 ease-linear;
  }

  .sidebar-tooltip {
    @apply absolute w-auto p-2 m-2 min-w-max left-14
    rounded-md shadow-md text-white bg-gray-900
    text-xs font-bold transition-all duration-100 scale-0
    origin-left;
  }
</style>
