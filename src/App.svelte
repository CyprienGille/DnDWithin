<script lang="ts">
  import CharacterSheet from "./lib/CharacterSheet.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Counter from "./lib/Counter.svelte";

  let promise = invoke("get_default");

  function open_file() {
    promise = invoke("open_file");
  }
</script>

<main>
  <div class="flex text-center">
    <div class="w-1/2">
      <button
        class="text-lg rounded-md bg-teal-200 text-slate-900 p-2 hover:bg-teal-400"
        type="button"
        on:click={open_file}>Open Character Sheet</button
      >
    </div>
    <div class="w-1/2">
      <button
        class="text-lg rounded-md bg-teal-200 text-slate-900 p-2 hover:bg-teal-400"
        type="button">Save Changes</button
      >
    </div>
  </div>
  {#await promise}
    Waiting For A Character Sheet...
  {:then c}
    <CharacterSheet {c} />
  {/await}
</main>
