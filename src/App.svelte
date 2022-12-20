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
    c.prof_st_str = Boolean(c.prof_st_str);
    c.prof_st_dex = Boolean(c.prof_st_dex);
    c.prof_st_con = Boolean(c.prof_st_con);
    c.prof_st_int = Boolean(c.prof_st_int);
    c.prof_st_wis = Boolean(c.prof_st_wis);
    c.prof_st_cha = Boolean(c.prof_st_cha);
    c.ac = Number(c.ac);
    c.inspi = Boolean(c.inspi);
    c.max_hp = Number(c.max_hp);
    c.hp = Number(c.hp);
    c.temp_hp = Number(c.temp_hp);
    c.ds_s_1 = Boolean(c.ds_s_1);
    c.ds_s_2 = Boolean(c.ds_s_2);
    c.ds_s_3 = Boolean(c.ds_s_3);
    c.ds_f_1 = Boolean(c.ds_f_1);
    c.ds_f_2 = Boolean(c.ds_f_2);
    c.ds_f_3 = Boolean(c.ds_f_3);
    c.cp = Number(c.cp);
    c.sp = Number(c.sp);
    c.ep = Number(c.ep);
    c.gp = Number(c.gp);
    c.pp = Number(c.pp);
    c.eq_1.qty = Number(c.eq_1.qty);
    c.eq_2.qty = Number(c.eq_2.qty);
    c.eq_3.qty = Number(c.eq_3.qty);
    c.eq_4.qty = Number(c.eq_4.qty);
    c.eq_5.qty = Number(c.eq_5.qty);
    c.eq_6.qty = Number(c.eq_6.qty);
    c.eq_7.qty = Number(c.eq_7.qty);
    c.eq_8.qty = Number(c.eq_8.qty);
    c.eq_9.qty = Number(c.eq_9.qty);
    c.eq_10.qty = Number(c.eq_10.qty);
    c.eq_11.qty = Number(c.eq_11.qty);
    c.eq_12.qty = Number(c.eq_12.qty);
    c.eq_13.qty = Number(c.eq_13.qty);
    c.eq_14.qty = Number(c.eq_14.qty);
    c.eq_15.qty = Number(c.eq_15.qty);
    c.eq_16.qty = Number(c.eq_16.qty);
    c.eq_17.qty = Number(c.eq_17.qty);
    c.eq_18.qty = Number(c.eq_18.qty);
    c.eq_19.qty = Number(c.eq_19.qty);
    c.eq_20.qty = Number(c.eq_20.qty);
    c.eq_21.qty = Number(c.eq_21.qty);
    c.eq_22.qty = Number(c.eq_22.qty);
    c.eq_23.qty = Number(c.eq_23.qty);
    c.eq_24.qty = Number(c.eq_24.qty);
    c.eq_25.qty = Number(c.eq_25.qty);
    c.eq_26.qty = Number(c.eq_26.qty);
    c.eq_27.qty = Number(c.eq_27.qty);
    c.eq_28.qty = Number(c.eq_28.qty);
    c.eq_29.qty = Number(c.eq_29.qty);
    c.eq_30.qty = Number(c.eq_30.qty);

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
