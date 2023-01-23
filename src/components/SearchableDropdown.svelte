<script lang="ts">
  import { clickOutside } from "./hooks/clickOutside";

  export let inputText: string = "";
  export let expand: boolean = true;
  export let placeholder: string = undefined;
  export let value: string = "";
  export let options: Array<{
    group: string;
    options: Array<{ text: string }>;
  }> = [];

  let listElement;
  let inputElement: HTMLInputElement;
  let filteredOptions: Array<{
    group: string;
    options: Array<{ text: string }>;
  }> = [];
  let isListOpen: boolean = false;
  let selectedOption: string;

  function filter() {
    const sanitized = inputText.trim().toLowerCase();

    let results = [];
    options.forEach((group) => {
      let groupResults = group.options.filter((o) =>
        o.text.toLowerCase().includes(sanitized)
      );
      if (groupResults.length > 0) {
        results.push({
          group: group.group,
          options: groupResults,
        });
      }
    });

    return results;
  }

  async function onInputKeyup(event: KeyboardEvent) {
    switch (event.key) {
      case "Escape":
      case "ArrowUp":
      case "ArrowLeft":
      case "ArrowRight":
      case "Enter":
      case "Tab":
      case "Shift":
        break;
      case "ArrowDown":
        showList();
        listElement
          .querySelector(`[role="option"]:not([aria-disabled="true"])`)
          ?.focus();

        event.preventDefault();
        event.stopPropagation();
        break;

      default:
        showList();
    }
  }

  function onInputKeydown(event: KeyboardEvent) {
    let flag = false;

    switch (event.key) {
      case "Escape":
        hideList(false);
        flag = true;
        break;

      case "Tab":
        hideList();
        break;
    }

    if (flag) {
      event.preventDefault();
      event.stopPropagation();
    }
  }

  function onInputClick() {
    showList();
  }

  function onOptionClick(event) {
    console.log("is this before or after");

    selectOption(event.target);
    hideList();
  }

  function onListKeyDown(event) {
    let flag = false;

    switch (event.key) {
      case "ArrowUp":
        let prevOptionElement = event.target.previousElementSibling;

        while (prevOptionElement) {
          if (
            prevOptionElement.matches(
              `[role="option"]:not([aria-disabled="true"])`
            )
          )
            break;
          prevOptionElement = prevOptionElement.previousElementSibling;
        }

        prevOptionElement?.focus();
        flag = true;
        break;

      case "ArrowDown":
        let nextOptionElement = event.target.nextElementSibling;

        while (nextOptionElement) {
          if (
            nextOptionElement.matches(
              `[role="option"]:not([aria-disabled="true"])`
            )
          )
            break;
          nextOptionElement = nextOptionElement.nextElementSibling;
        }

        nextOptionElement?.focus();
        flag = true;
        break;

      case "Enter":
        selectOption(event.target);
        hideList();
        flag = true;
        break;

      case "Escape":
        hideList(false);
        flag = true;
        break;

      case "Tab":
        hideList();
        break;

      default:
        inputElement.focus();
    }

    if (flag) {
      event.preventDefault();
      event.stopPropagation();
    }
  }

  function showList() {
    filteredOptions = inputText === "" ? options : filter();
    isListOpen = true;
  }

  function hideList(overrideCurrentText = true) {
    if (!isListOpen) return;

    if (selectedOption && overrideCurrentText) {
      inputElement.value = selectedOption;
    }

    isListOpen = false;
    inputElement.focus();
  }

  function selectOption(optionElement: {
    dataset: { value: string; text: string };
  }) {
    value = optionElement.dataset.value;
    selectedOption = optionElement.dataset.text;
    inputText = optionElement.dataset.text;
  }
</script>

<div class="bg-white flex flex-col rounded-md text-black">
  <div
    class="relative"
    use:clickOutside={["click"]}
    on:clickOutside={() => hideList(false)}
  >
    <input
      bind:this={inputElement}
      bind:value={inputText}
      on:input
      on:blur
      on:keyup={onInputKeyup}
      on:keydown
      on:keydown={onInputKeydown}
      on:mousedown={onInputClick}
      class="m-0 w-full p-2 rounded-md focus:outline-none"
      type="text"
      autocapitalize="none"
      autocomplete="off"
      {placeholder}
      spellcheck="false"
      role="combobox"
      aria-autocomplete="list"
      aria-expanded={isListOpen}
    />

    {#if filteredOptions.length > 0}
      <ul
        class="inset-y-10 h-max max-h-[230px] absolute bg-white min-w-full p-1 overflow-auto z-[100] rounded-md shadow-md"
        role="listbox"
        hidden={!isListOpen}
        on:click={onOptionClick}
        on:click
        on:keydown={onListKeyDown}
        bind:this={listElement}
      >
        {#each filteredOptions as optionGroup}
          <li class="text-sm text-gray-400 pl-1">
            {optionGroup.group}
          </li>
          {#each optionGroup.options as option}
            <li
              class="flex items-center justify-between p-1 hover:bg-gray-200 rounded-md focus:outline-none focus:bg-blue-300"
              role="option"
              tabindex={-1}
              data-text={option.text}
              data-value={option.text}
              aria-selected={value === option.text}
            >
              {option.text}
            </li>
          {/each}
        {/each}
      </ul>
    {/if}
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
