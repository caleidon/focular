<script lang="ts" context="module">
  export class ContextOption {
    text: string;
    onClick: () => void;
    constructor(text: string, onClick: () => void) {
      this.text = text;
      this.onClick = onClick;
    }
  }
</script>

<script lang="ts">
  export let option: ContextOption;
</script>

<div
  on:click={option.onClick}
  class="px-2 py-1 text-md flex items-center gap-1 text-white cursor-default circle rounded-lg hover:bg-gray-700 whitespace-nowrap"
>
  {option.text}
</div>
