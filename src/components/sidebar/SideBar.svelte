<script lang="ts">
  import { Link } from "svelte-navigator";
  import SideBarIcon from "./SideBarIcon.svelte";
  import fileDocument from "@iconify/icons-mdi/file-document";
  import tagIcon from "@iconify/icons-mdi/tag";
  import cogIcon from "@iconify/icons-mdi/cog";
  import folderPlus from "@iconify/icons-mdi/folder-plus";
  import Modal from "../Modal.svelte";
  import FolderPicker from "../FolderPicker.svelte";
  import { allFolders } from "../../code/valuesStore";

  let folderPickerModal;
</script>

<aside>
  <div
    class="grid grid-rows-[repeat(4,60px)] w-[65px] h-screen dark:bg-gray-900 shadow-lg"
  >
    <Link to="/">
      <SideBarIcon icon={fileDocument} tooltipText="All files" />
    </Link>

    <Link to="untagged">
      <SideBarIcon icon={tagIcon} tooltipText="Untagged files" />
    </Link>

    <Link to="preferences">
      <SideBarIcon icon={cogIcon} tooltipText="Preferences" />
    </Link>

    <div on:click={() => folderPickerModal.openModal()}>
      <SideBarIcon icon={folderPlus} tooltipText="Add folder" />
    </div>

    <Modal component={FolderPicker} bind:this={folderPickerModal} />

    <p>----------</p>

    {#each $allFolders as folder}{/each}
  </div>
</aside>

<style lang="postcss" global>
</style>
