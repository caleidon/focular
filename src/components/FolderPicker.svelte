<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { desktopDir } from "@tauri-apps/api/path";
  import { createEventDispatcher } from "svelte";
  import {
    addFolder,
    validateFolderName,
    validateFolderPath,
  } from "../code/FolderManager";

  const dispatch = createEventDispatcher();

  let folderName: string = "";
  let folderPath: string = "";

  let folderNameValid: boolean = true;
  let folderPathValid: boolean = true;

  async function openFileDialog() {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await desktopDir(),
    });
    if (selected && typeof selected === "string") {
      folderPath = selected;
      folderPathValid = validateFolderPath(folderPath) ? true : false;
    }
  }

  async function validateName() {
    folderNameValid = validateFolderName(folderName) ? true : false;
  }
</script>

<div class="p-4 rounded-md bg-gray-800 text-white">
  <h3 class="font-bold text-lg">Add folder</h3>
  <input
    type="text"
    placeholder="Folder name"
    class="input input-bordered w-full max-w-xs my-1"
    bind:value={folderName}
    on:input={validateName}
  />
  {#if !folderNameValid}
    <p class="text-red-600">Folder name is already taken</p>
  {/if}

  <button class="btn" on:click={openFileDialog}>Select folder</button>
  {#if folderPath !== ""}
    <p class=" text-white">
      Chosen folder: {folderPath}
    </p>
  {/if}
  {#if !folderPathValid}
    <p class="text-red-600">
      This folder already exists or is a child of an existing folder
    </p>
  {/if}

  <div class="flex flex-row justify-end ">
    <button class="btn btn-error" on:click={() => dispatch("closeModal")}
      >Cancel</button
    >
    <button
      on:click={() => {
        addFolder(folderName, folderPath);
        dispatch("closeModal");
      }}
      disabled={!folderNameValid ||
        !folderPathValid ||
        folderName === "" ||
        folderPath === ""}
      class="btn ml-2 btn-primary">Create</button
    >
  </div>
</div>
