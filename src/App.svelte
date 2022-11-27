<script lang="ts">
  import CharacterSheet from "./lib/CharacterSheet.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let promise = invoke("get_default");

  function open_file() {
    promise = invoke("open_file");
  }

  async function save_file(promise) {
    let c = await promise;

    c.exp = Number(c.exp);
    c.str = Number(c.str);
    c.dex = Number(c.dex);
    c.con = Number(c.con);
    c.int = Number(c.int);
    c.wis = Number(c.wis);
    c.cha = Number(c.cha);
    c.prof_mod = Number(c.prof_mod);
    c.st_str = Number(c.st_str);
    c.st_dex = Number(c.st_dex);
    c.st_con = Number(c.st_con);
    c.st_int = Number(c.st_int);
    c.st_wis = Number(c.st_wis);
    c.st_cha = Number(c.st_cha);
    c.prof_st_str = Boolean(c.prof_st_str);
    c.prof_st_dex = Boolean(c.prof_st_dex);
    c.prof_st_con = Boolean(c.prof_st_con);
    c.prof_st_int = Boolean(c.prof_st_int);
    c.prof_st_wis = Boolean(c.prof_st_wis);
    c.prof_st_cha = Boolean(c.prof_st_cha);
    c.ac = Number(c.ac);
    c.inspi = Boolean(c.inspi);

    invoke("save_character_to_file", { c: c });
  }
</script>

<main>
  <div class="flex text-center">
    <div class="w-1/2">
      <button
        class="w-full text-lg bg-blue-200 text-slate-900 p-2 hover:bg-teal-400"
        type="button"
        on:click={open_file}>Open Character Sheet</button
      >
    </div>
    <div class="w-1/2">
      <button
        class="w-full text-lg bg-blue-200 text-slate-900 p-2 hover:bg-teal-400"
        type="button"
        on:click={() => save_file(promise)}>Save Changes</button
      >
    </div>
  </div>
  {#await promise}
    <div class="font-mono text-xl text-center py-10">
      Waiting For A Character Sheet...
    </div>
  {:then c}
    <CharacterSheet {c} />
  {/await}
</main>
