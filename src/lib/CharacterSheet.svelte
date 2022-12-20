<script lang="ts">
  export let c;

  let adding_spell = false;

  let new_spell;

  let prof_options = [
    { mult: 0.0, text: "" },
    { mult: 0.5, text: "H" },
    { mult: 1.0, text: "P" },
    { mult: 2.0, text: "E" },
  ];

  let ab_options = [
    "Strength",
    "Dexterity",
    "Constitution",
    "Intelligence",
    "Wisdom",
    "Charisma",
  ];

  function compute_modifier(score, prof_mult) {
    if (typeof prof_mult === "string") {
      if (prof_mult === "false") {
        prof_mult = 0;
      } else {
        prof_mult = 1;
      }
    }
    const num =
      Math.floor((score - 10) / 2) + Math.floor(prof_mult * c.prof_mod);
    return (num >= 0 ? "+" : "") + num;
  }

  function init_new_spell() {
    new_spell = {
      prep: "N",
      name: "Name",
      save: "DEX 10",
      time: "1A",
      range: "Touch",
      comp: "VSM",
      duration: "Instant",
      page_ref: "PHB 255",
      notes: "Ritual",
    };
  }

  function open_spell_creation() {
    init_new_spell();
    adding_spell = true;
  }

  function close_spell_creation() {
    c.spells.push(JSON.parse(JSON.stringify(new_spell)));
    c.spells = c.spells;
    adding_spell = false;
  }
</script>

<main>
  <div class="w-full flex items-center py-4 border-2">
    <div class="w-1/3 px-1">
      <input
        class="rounded-md w-full py-2 px-2 border-slate-400 border-2"
        type="text"
        name="CName"
        id="cname"
        bind:value={c.name}
      />
    </div>
    <div class="w-2/3">
      <div class="flex">
        <div class="w-1/2">
          <label for="classlevel" class="text-sm">Class and Level</label>
          <input
            class="rounded-md p-2 border-slate-400 border-2"
            type="text"
            id="classlevel"
            bind:value={c.class_level}
          />
        </div>
        <div class="w-1/2">
          <label for="playername" class="text-sm">Player Name</label>
          <input
            class="rounded-md p-2 border-slate-400 border-2"
            type="text"
            id="playername"
            bind:value={c.player_name}
          />
        </div>
      </div>
      <div class="flex items-center">
        <div class="w-1/3">
          <label class="text-sm" for="race">Race</label>
          <input
            class="rounded-md p-2 border-slate-400 border-2"
            type="text"
            id="race"
            bind:value={c.race}
          />
        </div>
        <div class="w-1/3">
          <label class="text-sm" for="background"> Background </label>
          <input
            class="rounded-md p-2 border-slate-400 border-2"
            type="text"
            id="background"
            bind:value={c.bg}
          />
        </div>
        <div class="w-1/3">
          <label class="text-sm" for="exp"> Experience Points </label>
          <input
            class="rounded-md p-2 border-slate-400 border-2"
            type="text"
            id="exp"
            bind:value={c.exp}
          />
        </div>
      </div>
    </div>
  </div>
  <div class="flex mb-6">
    <div class="w-1/6 bg-slate-300 rounded-md pb-11">
      <div class="h-1/6 mb-2">
        <div class="text-center font-semibold">STR</div>
        <div class="text-center font-bold text-lg">
          {compute_modifier(c.str, 0)}
        </div>
        <div class="text-center">
          <input
            class="text-center w-11/12 border-slate-400 border-2 rounded-sm"
            type="text"
            bind:value={c.str}
          />
        </div>
      </div>
      <div class="h-1/6 mb-2">
        <div class="text-center font-semibold">DEX</div>
        <div class="text-center font-bold text-lg">
          {compute_modifier(c.dex, 0)}
        </div>
        <div class="text-center">
          <input
            class="text-center w-11/12 border-slate-400 border-2 rounded-sm"
            type="text"
            bind:value={c.dex}
          />
        </div>
      </div>
      <div class="h-1/6 mb-2">
        <div class="text-center font-semibold">CON</div>
        <div class="text-center font-bold text-lg">
          {compute_modifier(c.con, 0)}
        </div>
        <div class="text-center">
          <input
            class="text-center w-11/12 border-slate-400 border-2 rounded-sm"
            type="text"
            bind:value={c.con}
          />
        </div>
      </div>
      <div class="h-1/6 mb-2">
        <div class="text-center font-semibold">INT</div>
        <div class="text-center font-bold text-lg">
          {compute_modifier(c.int, 0)}
        </div>
        <div class="text-center">
          <input
            class="text-center w-11/12 border-slate-400 border-2 rounded-sm"
            type="text"
            bind:value={c.int}
          />
        </div>
      </div>
      <div class="h-1/6 mb-2">
        <div class="text-center font-semibold">WIS</div>
        <div class="text-center font-bold text-lg">
          {compute_modifier(c.wis, 0)}
        </div>
        <div class="text-center">
          <input
            class="text-center w-11/12 border-slate-400 border-2 rounded-sm"
            type="text"
            bind:value={c.wis}
          />
        </div>
      </div>
      <div class="h-1/6 mb-1">
        <div class="text-center font-semibold">CHA</div>
        <div class="text-center font-bold text-lg">
          {compute_modifier(c.cha, 0)}
        </div>
        <div class="text-center">
          <input
            class="text-center w-11/12 border-slate-400 border-2 rounded-sm"
            type="text"
            bind:value={c.cha}
          />
        </div>
      </div>
    </div>
    <div class="w-1/6">
      <div class="h-1/3 mb-1">
        <div class="font-semibold text-center mb-1">Saving throws</div>
        <div class="text-xs flex ml-1">
          <input type="checkbox" bind:checked={c.prof_st_str} />
          <div class="rounded-md border-2 mx-2 px-1 w-8">
            {compute_modifier(c.str, c.prof_st_str)}
          </div>
          <div>Strength</div>
        </div>
        <div class="text-xs flex ml-1">
          <input type="checkbox" bind:checked={c.prof_st_dex} />
          <div class="rounded-md border-2 mx-2 px-1 w-8">
            {compute_modifier(c.dex, c.prof_st_dex)}
          </div>
          <div>Dexterity</div>
        </div>
        <div class="text-xs flex ml-1">
          <input type="checkbox" bind:checked={c.prof_st_con} />
          <div class="rounded-md border-2 mx-2 px-1 w-8">
            {compute_modifier(c.con, c.prof_st_con)}
          </div>
          <div>Constitution</div>
        </div>
        <div class="text-xs flex ml-1">
          <input type="checkbox" bind:checked={c.prof_st_int} />
          <div class="rounded-md border-2 mx-2 px-1 w-8">
            {compute_modifier(c.int, c.prof_st_int)}
          </div>
          <div>Intelligence</div>
        </div>
        <div class="text-xs flex ml-1">
          <input type="checkbox" bind:checked={c.prof_st_wis} />
          <div class="rounded-md border-2 mx-2 px-1 w-8">
            {compute_modifier(c.wis, c.prof_st_wis)}
          </div>
          <div>Wisdom</div>
        </div>
        <div class="text-xs flex ml-1">
          <input type="checkbox" bind:checked={c.prof_st_cha} />
          <div class="rounded-md border-2 mx-2 px-1 w-8">
            {compute_modifier(c.cha, c.prof_st_cha)}
          </div>
          <div>Charisma</div>
        </div>
        <div>
          <div class="text-xs font-semibold text-center mb-1">
            Saving Throws Modifiers
          </div>
          <textarea class="w-full text-xs" bind:value={c.st_mods} />
        </div>
      </div>
      <div class="h-2/3 mt-6">
        <div class="font-semibold text-center">Skills</div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.acro}>
              {#each prof_options as option}
                <option value={option}>
                  {option.text}
                </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.dex, c.acro.mult)}
          </div>
          <div>Acrobatics (DEX)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.anim}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.wis, c.anim.mult)}
          </div>
          <div>Animal Handling (WIS)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.arca}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.int, c.arca.mult)}
          </div>
          <div>Arcana (INT)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.athl}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.str, c.athl.mult)}
          </div>
          <div>Athletics (STR)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.dece}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.cha, c.dece.mult)}
          </div>
          <div>Deception (CHA)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.hist}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.int, c.hist.mult)}
          </div>
          <div>History (INT)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.insi}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.wis, c.insi.mult)}
          </div>
          <div>Insight (WIS)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.inti}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.cha, c.inti.mult)}
          </div>
          <div>Intimidation (CHA)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.inve}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.int, c.inve.mult)}
          </div>
          <div>Investigation (INT)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.medi}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.wis, c.medi.mult)}
          </div>
          <div>Medicine (WIS)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.natu}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.int, c.natu.mult)}
          </div>
          <div>Nature (INT)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.perc}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.wis, c.perc.mult)}
          </div>
          <div>Perception (WIS)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.perf}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.cha, c.perf.mult)}
          </div>
          <div>Performance (CHA)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.pers}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.cha, c.pers.mult)}
          </div>
          <div>Persuasion (CHA)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.reli}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.int, c.reli.mult)}
          </div>
          <div>Religion (INT)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.slei}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.dex, c.slei.mult)}
          </div>
          <div>Sleight of Hand (DEX)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.stea}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.dex, c.stea.mult)}
          </div>
          <div>Stealth (DEX)</div>
        </div>
        <div class="flex text-xs">
          <form>
            <select bind:value={c.surv}>
              {#each prof_options as option}
                <option value={option}> {option.text} </option>
              {/each}
            </select>
          </form>
          <div class="rounded-md border-2 mx-1 px-1 w-8">
            {compute_modifier(c.wis, c.surv.mult)}
          </div>
          <div>Survival (WIS)</div>
        </div>
      </div>
    </div>
    <div class="w-2/6">
      <div class="h-2/3 ">
        <div class="h-1/6 flex">
          <div class="w-1/2 text-center border-2 mt-1">
            <div class="font-semibold">Initiative</div>
            <div>{compute_modifier(c.dex, 0)}</div>
          </div>
          <div class="w-1/2 text-center border-2 mt-1">
            <div class="font-semibold">Armor Class</div>
            <input class="text-center w-1/2" type="text" bind:value={c.ac} />
          </div>
        </div>
        <div class="h-2/6 mb-1 border-2">
          <div class="font-semibold text-center">Resistances & Immunities</div>
          <textarea class="w-full text-sm h-2/3" bind:value={c.res} />
        </div>
        <div class="h-1/6 flex mt-1">
          <div class="w-1/2 text-right pr-1">
            <input type="checkbox" bind:checked={c.inspi} />
          </div>
          <div class="w-1/2 text-left">Inspiration</div>
        </div>
        <div class="h-1/6 flex">
          <div class="text-sm w-1/2 h-1/2 text-center">Proficiency Bonus</div>
          <input
            class="text-sm w-1/2 h-1/2 text-center border-2"
            type="text"
            bind:value={c.prof_mod}
          />
        </div>
        <div class="h-1/6 border-2">
          <div class="font-semibold text-center">Speeds</div>
          <input class="text-sm w-full" type="text" bind:value={c.speeds} />
        </div>
      </div>
      <div class="h-1/3 bg-slate-200">
        <div class="font-semibold pl-2">Proficiencies & Languages</div>
        <textarea
          class="w-full h-full bg-slate-200 text-sm"
          bind:value={c.profs}
        />
      </div>
    </div>
    <div class="w-2/6">
      <div class="h-1/5 mb-4">
        <div class="h-1/2 flex">
          <div class="w-1/3">
            <div class="font-semibold pl-1">Max HP</div>
            <input
              class="w-full text-center"
              type="text"
              bind:value={c.max_hp}
            />
          </div>
          <div class="w-1/3">
            <div class="font-semibold pl-1">Current HP</div>

            <input class="w-full text-center" type="text" bind:value={c.hp} />
          </div>
          <div class="w-1/3">
            <div class="font-semibold pl-1">Temp HP</div>

            <input
              class="w-full text-center"
              type="text"
              bind:value={c.temp_hp}
            />
          </div>
        </div>
        <div class="h-1/2 flex">
          <div class="w-1/2">
            <div class="font-semibold">Hit Dice</div>
            <div class="text-sm flex">
              <div class="w-1/2 text-center">Total</div>
              <input
                class="w-1/2 border-2"
                type="text"
                bind:value={c.hd_total}
              />
            </div>
            <div class="text-sm flex">
              <div class="w-1/2 text-center">Used</div>
              <input class="w-1/2 border-2" type="text" bind:value={c.hd} />
            </div>
          </div>
          <div class="w-1/2">
            <div class="h-1/3 font-semibold">Death Saves</div>
            <div class="h-1/3 flex">
              <div class="w-1/4 text-sm ml-1">Success</div>
              <input class="w-1/4" type="checkbox" bind:checked={c.ds_s_1} />
              <input class="w-1/4" type="checkbox" bind:checked={c.ds_s_2} />
              <input class="w-1/4" type="checkbox" bind:checked={c.ds_s_3} />
            </div>
            <div class="h-1/3 flex">
              <div class="w-1/4 text-sm ml-1">Failure</div>
              <input class="w-1/4" type="checkbox" bind:checked={c.ds_f_1} />
              <input class="w-1/4" type="checkbox" bind:checked={c.ds_f_2} />
              <input class="w-1/4" type="checkbox" bind:checked={c.ds_f_3} />
            </div>
          </div>
        </div>
      </div>
      <div class="h-4/5 border-2 mb-10">
        <div class="font-semibold text-center">Actions & Reactions</div>
        <textarea class="h-full w-full" bind:value={c.actions} />
      </div>
    </div>
  </div>
  <div class="flex items-center">
    <div class="w-2/6 border-2">
      <div class="text-center font-semibold">Senses</div>
      <div class="h-1/5 flex">
        <div class="w-1/4 text-center">
          {10 + Number(compute_modifier(c.wis, c.perc.mult))}
        </div>
        <div class="w-3/4">Passive Perception</div>
      </div>
      <div class="h-1/5 flex">
        <div class="w-1/4 text-center">
          {10 + Number(compute_modifier(c.wis, c.insi.mult))}
        </div>
        <div class="w-3/4">Passive Insight</div>
      </div>
      <div class="h-1/5 flex">
        <div class="w-1/4 text-center">
          {10 + Number(compute_modifier(c.int, c.inve.mult))}
        </div>
        <div class="w-3/4">Passive Investigation</div>
      </div>
      <div class="h-2/5">
        <textarea class="w-full h-full" bind:value={c.senses} />
      </div>
    </div>
    <div class="w-4/6">
      <div class="text-center font-semibold">Attacks & Cantrips</div>
      <div class="flex font-semibold h-full">
        <div class="w-4/12">NAME</div>
        <div class="w-1/12">HIT</div>
        <div class="w-4/12">DAMAGE/TYPE</div>
        <div class="w-3/12">NOTES</div>
      </div>
      <div class="flex h-full">
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_1.name} />
        </div>
        <div class="w-1/12">
          <textarea class="w-full" bind:value={c.atk_1.hit} />
        </div>
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_1.dmg} />
        </div>
        <div class="w-3/12">
          <textarea class="w-full" bind:value={c.atk_1.notes} />
        </div>
      </div>
      <div class="flex h-full">
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_2.name} />
        </div>
        <div class="w-1/12">
          <textarea class="w-full" bind:value={c.atk_2.hit} />
        </div>
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_2.dmg} />
        </div>
        <div class="w-3/12">
          <textarea class="w-full" bind:value={c.atk_2.notes} />
        </div>
      </div>
      <div class="flex h-full">
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_3.name} />
        </div>
        <div class="w-1/12">
          <textarea class="w-full" bind:value={c.atk_3.hit} />
        </div>
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_3.dmg} />
        </div>
        <div class="w-3/12">
          <textarea class="w-full" bind:value={c.atk_3.notes} />
        </div>
      </div>
      <div class="flex h-full">
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_4.name} />
        </div>
        <div class="w-1/12">
          <textarea class="w-full" bind:value={c.atk_4.hit} />
        </div>
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_4.dmg} />
        </div>
        <div class="w-3/12">
          <textarea class="w-full" bind:value={c.atk_4.notes} />
        </div>
      </div>
      <div class="flex h-full">
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_5.name} />
        </div>
        <div class="w-1/12">
          <textarea class="w-full" bind:value={c.atk_5.hit} />
        </div>
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_5.dmg} />
        </div>
        <div class="w-3/12">
          <textarea class="w-full" bind:value={c.atk_5.notes} />
        </div>
      </div>
      <div class="flex h-full">
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_6.name} />
        </div>
        <div class="w-1/12">
          <textarea class="w-full" bind:value={c.atk_6.hit} />
        </div>
        <div class="w-4/12">
          <textarea class="w-full" bind:value={c.atk_6.dmg} />
        </div>
        <div class="w-3/12">
          <textarea class="w-full" bind:value={c.atk_6.notes} />
        </div>
      </div>
    </div>
  </div>
  <div class="w-full border-2">
    <div class="text-center font-semibold">Features & Traits</div>
    <div class="flex">
      <div class="w-1/3">
        <textarea
          class="w-full pb-48 h-full text-sm"
          bind:value={c.features_1}
        />
      </div>
      <div class="w-1/3">
        <textarea
          class="w-full pb-48 h-full text-sm"
          bind:value={c.features_2}
        />
      </div>
      <div class="w-1/3">
        <textarea
          class="w-full pb-48 h-full text-sm"
          bind:value={c.features_3}
        />
      </div>
    </div>
  </div>
  <div class="w-full text-center">
    <div class="text-center font-semibold">Equipment</div>
    <div class="flex">
      <div class="w-2/12 border-2">
        <div class="h-1/5 flex">
          <div class="w-1/2">Copper</div>
          <div class="w-1/2">
            <input
              class="w-11/12 text-center"
              type="number"
              bind:value={c.cp}
            />
          </div>
        </div>
        <div class="h-1/5 flex">
          <div class="w-1/2">Silver</div>
          <div class="w-1/2">
            <input
              class="w-11/12 text-center"
              type="number"
              bind:value={c.sp}
            />
          </div>
        </div>
        <div class="h-1/5 flex">
          <div class="w-1/2">Electrum</div>
          <div class="w-1/2">
            <input
              class="w-11/12 text-center"
              type="number"
              bind:value={c.ep}
            />
          </div>
        </div>
        <div class="h-1/5 flex">
          <div class="w-1/2">Gold</div>
          <div class="w-1/2">
            <input
              class="w-11/12 text-center"
              type="number"
              bind:value={c.gp}
            />
          </div>
        </div>
        <div class="h-1/5 flex">
          <div class="w-1/2">Platinum</div>
          <div class="w-1/2">
            <input
              class="w-11/12 text-center"
              type="number"
              bind:value={c.pp}
            />
          </div>
        </div>
      </div>
      <div class="w-5/12 border-2">
        <div class="flex">
          <div class="w-8/12 font-semibold">Name</div>
          <div class="w-2/12 font-semibold">Quantity</div>
          <div class="w-2/12 font-semibold">Weight</div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_1.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_1.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_1.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_2.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_2.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_2.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_3.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_3.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_3.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_4.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_4.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_4.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_5.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_5.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_5.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_6.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_6.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_6.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_7.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_7.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_7.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_8.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_8.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_8.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_9.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_9.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_9.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_10.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_10.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_10.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_11.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_11.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_11.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_12.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_12.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_12.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_13.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_13.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_13.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_14.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_14.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_14.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_15.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_15.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_15.w} />
          </div>
        </div>
      </div>
      <div class="w-5/12 border-2">
        <div class="flex">
          <div class="w-8/12 font-semibold">Name</div>
          <div class="w-2/12 font-semibold">Quantity</div>
          <div class="w-2/12 font-semibold">Weight</div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_16.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_16.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_16.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_17.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_17.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_17.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_18.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_18.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_18.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_19.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_19.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_19.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_20.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_20.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_20.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_21.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_21.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_21.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_22.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_22.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_22.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_23.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_23.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_23.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_24.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_24.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_24.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_25.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_25.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_25.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_26.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_26.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_26.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_27.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_27.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_27.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_28.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_28.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_28.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_29.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_29.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_29.w} />
          </div>
        </div>
        <div class="flex">
          <div class="w-8/12">
            <input type="text" bind:value={c.eq_30.name} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="number" bind:value={c.eq_30.qty} />
          </div>
          <div class="w-2/12">
            <input class="w-full" type="text" bind:value={c.eq_30.w} />
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="w-full text-center border-2">
    <div class="font-semibold">Characteristics</div>
    <div class="flex">
      <div class="w-1/5">Gender</div>
      <div class="w-1/5">Age</div>
      <div class="w-1/5">Size</div>
      <div class="w-1/5">Height</div>
      <div class="w-1/5">Weight</div>
    </div>
    <div class="flex">
      <textarea class="w-1/5" bind:value={c.gender} />
      <textarea class="w-1/5" bind:value={c.age} />
      <textarea class="w-1/5" bind:value={c.size} />
      <textarea class="w-1/5" bind:value={c.height} />
      <textarea class="w-1/5" bind:value={c.weight} />
    </div>
    <div class="flex">
      <div class="w-1/5">Alignment</div>
      <div class="w-1/5">Faith</div>
      <div class="w-1/5">Skin</div>
      <div class="w-1/5">Eyes</div>
      <div class="w-1/5">Hair</div>
    </div>
    <div class="flex">
      <textarea class="w-1/5" bind:value={c.alig} />
      <textarea class="w-1/5" bind:value={c.faith} />
      <textarea class="w-1/5" bind:value={c.skin} />
      <textarea class="w-1/5" bind:value={c.eyes} />
      <textarea class="w-1/5" bind:value={c.hair} />
    </div>
  </div>
  <div class="flex text-center border-2">
    <div class="w-1/3">
      <div class="font-semibold">Appearance</div>
      <textarea class="w-full" rows="15" bind:value={c.appearance} />
    </div>
    <div class="w-1/3">
      <div class="font-semibold">Allies and Organizations</div>
      <textarea class="w-full" rows="15" bind:value={c.allies} />
    </div>
    <div class="w-1/3">
      <div class="font-semibold">Personality Traits</div>
      <textarea class="w-full" rows="3" bind:value={c.p_traits} />
      <div class="font-semibold">Ideals</div>
      <textarea class="w-full" rows="3" bind:value={c.ideals} />
      <div class="font-semibold">Bonds</div>
      <textarea class="w-full" rows="3" bind:value={c.bonds} />
      <div class="font-semibold">Flaws</div>
      <textarea class="w-full" rows="3" bind:value={c.flaws} />
    </div>
  </div>
  <div class="flex text-center">
    <div class="w-1/3 border-2">
      <div class="font-semibold">Backstory</div>
      <textarea class="w-full" rows="20" bind:value={c.backstory} />
    </div>
    <div class="w-2/3 border-2">
      <div class="font-semibold">Notes</div>
      <div class="flex">
        <textarea class="w-1/2" rows="20" bind:value={c.notes_1} />
        <textarea class="w-1/2" rows="20" bind:value={c.notes_2} />
      </div>
    </div>
  </div>
  <div class="flex font-semibold text-center">
    <div class="w-6/12">Spellcasting Class</div>
    <div class="w-2/12">Spellcasting Ability</div>
    <div class="w-2/12">Spell Save DC</div>
    <div class="w-2/12">Spell Attack Bonus</div>
  </div>
  <div class="flex text-center">
    <textarea class="w-6/12" bind:value={c.sp_class} />
    <div class="w-2/12">
      <form>
        <select bind:value={c.sp_ab}>
          {#each ab_options as option}
            <option value={option}>{option}</option>
          {/each}
        </select>
      </form>
    </div>
    <textarea class="w-2/12" bind:value={c.sp_dc} />
    <textarea class="w-2/12" bind:value={c.sp_atk} />
  </div>
  <div class="text-center font-bold">Spells</div>
  <div class="flex font-semibold">
    <div class="w-1/12">Prepared</div>
    <div class="w-3/12">Name</div>
    <div class="w-1/12">Save/ATK</div>
    <div class="w-1/12">Time</div>
    <div class="w-1/12">Range</div>
    <div class="w-1/12">Components</div>
    <div class="w-1/12">Duration</div>
    <div class="w-1/12">Page Ref</div>
    <div class="w-1/12">Notes</div>
  </div>
  {#each c.spells as spell (spell.name)}
    <div class="flex">
      <div class="w-1/12">{spell.prep}</div>
      <div class="w-3/12">{spell.name}</div>
      <div class="w-1/12">{spell.save}</div>
      <div class="w-1/12">{spell.time}</div>
      <div class="w-1/12">{spell.range}</div>
      <div class="w-1/12">{spell.comp}</div>
      <div class="w-1/12">{spell.duration}</div>
      <div class="w-1/12">{spell.page_ref}</div>
      <div class="w-1/12">{spell.notes}</div>
    </div>
  {/each}

  {#if adding_spell}
    <div class="flex border-2">
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.prep} />
      </div>
      <div class="w-3/12">
        <input class="w-full" type="text" bind:value={new_spell.name} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.save} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.time} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.range} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.comp} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.duration} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.page_ref} />
      </div>
      <div class="w-1/12">
        <input class="w-full" type="text" bind:value={new_spell.notes} />
      </div>
    </div>

    <div class="text-center">
      <button
        class="w-full text-lg bg-blue-200 text-slate-900 p-2 hover:bg-teal-400"
        on:click={() => close_spell_creation()}>Done</button
      >
    </div>
  {:else}
    <div class="text-center">
      <button
        class="w-full text-lg bg-blue-200 text-slate-900 p-2 hover:bg-teal-400"
        on:click={() => open_spell_creation()}>Add Spell</button
      >
    </div>
  {/if}
</main>
