<script lang="ts" context="module">
  export class ContextMenuDivider {
    thickness: number;
    constructor(thickness: number) {
      this.thickness = thickness;
    }
  }
</script>

<script lang="ts">
  export let option: ContextMenuDivider;
</script>

<hr
  class="border-solid border-gray-700"
  style="border-width: {option.thickness}px"
/>
