<script>
  // This example is copied from
  // https://dev.to/vibhanshu909/how-to-create-a-full-featured-modal-component-in-svelte-and-trap-focus-within-474i
  import Modal from "../components/Modal.svelte";
  import TestComp from "../components/TestComp.svelte";

  let preferencesModal;
</script>

<button on:click={() => preferencesModal.openModal()}>OPEN ME</button>
<Modal component={TestComp} bind:this={preferencesModal} />
