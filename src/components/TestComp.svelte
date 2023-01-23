<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { updatePreferences } from "../code/ContentManager";
  import { preferences } from "../code/settingsStore";

  const dispatch = createEventDispatcher();

  enum SelectedPreferencesTab {
    General = 0,
    Appearance = 1,
    Behavior = 2,
    Advanced = 3,
  }

  let selectedPreferencesTab: SelectedPreferencesTab =
    SelectedPreferencesTab.General;

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      dispatch("closeModal");
    }
  }
</script>

<div class="w-[90vw] h-[90vh] max-w-[1000px]" on:keydown={handleKeydown}>
  <div class="flex flex-row h-full">
    <div class="flex flex-col">
      <ul
        class="flex-grow menu rounded-l-lg p-4 overflow-y-auto w-60 bg-base-100 text-base-content"
      >
        <p class="text-lg">Preferences</p>
        <li
          on:click={() =>
            (selectedPreferencesTab = SelectedPreferencesTab.General)}
        >
          <a>General</a>
        </li>
        <li
          on:click={() =>
            (selectedPreferencesTab = SelectedPreferencesTab.Appearance)}
        >
          <a>Appearance</a>
        </li>
        <li
          on:click={() =>
            (selectedPreferencesTab = SelectedPreferencesTab.Behavior)}
        >
          <a>Behavior</a>
        </li>
        <li
          on:click={() =>
            (selectedPreferencesTab = SelectedPreferencesTab.Advanced)}
        >
          <a>Advanced</a>
        </li>
      </ul>
    </div>

    <div
      class="flex-grow bg-gray-600 rounded-r-lg flex flex-row justify-between"
    >
      <div class="flex flex-col">
        {#if selectedPreferencesTab === SelectedPreferencesTab.General}
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Show file extensions</span>
              <input
                type="checkbox"
                bind:checked={$preferences.show_file_extensions}
                on:change={updatePreferences}
                class="checkbox checkbox-primary"
              />
            </label>
          </div>
        {/if}

        {#if selectedPreferencesTab === SelectedPreferencesTab.Appearance}
          <p>Appearance</p>
        {/if}

        {#if selectedPreferencesTab === SelectedPreferencesTab.Behavior}
          <p>Behavior</p>
        {/if}

        {#if selectedPreferencesTab === SelectedPreferencesTab.Advanced}
          <p>Advanced</p>
        {/if}
      </div>

      <div
        class="bg-red-300 h-8 w-8 cursor-pointer"
        on:click={() => dispatch("closeModal")}
      />
    </div>
  </div>
</div>
