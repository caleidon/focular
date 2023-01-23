<script lang="ts">
  import { booleanStore } from "./booleanStore";

  export let component;

  const { isOpen, open, close } = booleanStore(false);

  let dialogNode: HTMLDivElement;
  let lastFocus: Element;
  let ignoresFocusChange: boolean;

  function modalAction(node) {
    // mix of https://github.com/KittyGiraudel/focusable-selectors/blob/799829e3b8c329d679b3b414b5dfcfa257a817cf/index.js
    // and https://github.com/focus-trap/tabbable/blob/baa8c3044fe0a8fd8c0826f4a3e284872e1467a5/src/index.js#L1-L13
    const focusableCandidateSelectors =
      'a[href]:not([tabindex^="-"])' +
      ',area[href]:not([tabindex^="-"])' +
      ',input:not([type="hidden"]):not([type="radio"]):not([disabled]):not([tabindex^="-"])' +
      ',input[type="radio"]:not([disabled]):not([tabindex^="-"])' +
      ',select:not([disabled]):not([tabindex^="-"])' +
      ',textarea:not([disabled]):not([tabindex^="-"])' +
      ',button:not([disabled]):not([tabindex^="-"])' +
      ',iframe:not([tabindex^="-"])' +
      ',audio[controls]:not([tabindex^="-"])' +
      ',video[controls]:not([tabindex^="-"])' +
      ',[contenteditable]:not([tabindex^="-"])' +
      ',[tabindex]:not([tabindex^="-"])' +
      ',details>summary:not([tabindex^="-"])' +
      ',details:not([tabindex^="-"])';

    const attemptFocus = (element) => {
      ignoresFocusChange = true;
      try {
        element.focus();
      } catch {}
      ignoresFocusChange = false;
      return element === document.activeElement;
    };

    function focusFirstDescendant(element) {
      if (element) {
        const descendants = element.querySelectorAll(
          focusableCandidateSelectors
        );
        for (const el of descendants) {
          if (attemptFocus(el)) break;
        }
      }
    }

    function focusLastDescendant(element) {
      if (element) {
        const descendants = element.querySelectorAll(
          focusableCandidateSelectors
        );
        for (let i = descendants.length - 1; i >= 0; i--) {
          if (attemptFocus(descendants[i])) break;
        }
      }
    }

    function trapFocus(event) {
      if (!ignoresFocusChange) {
        if (dialogNode.contains(event.target)) {
          lastFocus = event.target;
        } else {
          focusFirstDescendant(dialogNode);
          if (lastFocus === document.activeElement) {
            focusLastDescendant(dialogNode);
          }
          lastFocus = document.activeElement;
        }
      }
    }

    function keydown(e: KeyboardEvent) {
      e.stopPropagation();
      if (e.key === "Escape") {
        close();
      }
    }

    let originalOverflow: string;
    // for accessibility
    if (document.body.style.overflow !== "hidden") {
      originalOverflow = document.body.style.overflow;
      document.body.style.overflow = "hidden";
    }
    document.addEventListener("focus", trapFocus, true);
    node.addEventListener("keydown", keydown);
    focusFirstDescendant(node);
    return {
      destroy() {
        if (originalOverflow) {
          document.body.style.overflow = originalOverflow;
        }
        document.removeEventListener("focus", trapFocus, true);
        node.removeEventListener("keydown", keydown);
      },
    };
  }

  export function openModal() {
    open();
  }
</script>

{#if $isOpen}
  <div tabindex="0" />
  <div
    class="absolute w-full h-full backdrop-blur-sm backdrop-brightness-75 flex text-black items-center justify-center"
    bind:this={dialogNode}
    use:modalAction
  >
    <div class="z-[10]" tabindex="0">
      <svelte:component this={component} on:closeModal={close} />
    </div>
    <div
      class="absolute w-full h-full bg-red-500 opacity-20"
      on:click={close}
    />
  </div>
  <div tabindex="0" />
{/if}
