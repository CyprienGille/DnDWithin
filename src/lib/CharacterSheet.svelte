<script lang="ts">
  export let c;

  console.log(c.acro);

  let prof_options = [
    { mult: 0.0, text: "" },
    { mult: 0.5, text: "H" },
    { mult: 1.0, text: "P" },
    { mult: 2.0, text: "E" },
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
      <div class="h-4/5 bg-slate-300">
        <div class="font-semibold text-center">Actions & Reactions</div>
        <textarea class="h-full w-full bg-slate-300" bind:value={c.actions} />
      </div>
    </div>
  </div>
  <div class="flex items-center">
    <div class="w-2/6">senses</div>
    <div class="w-4/6">attacks and cantrips</div>
  </div>
  <div class="w-full text-center">Features and traits</div>
  <div class="w-full text-center">Equipment</div>
  <div class="w-full text-center">Characteristics</div>
  <div class="flex text-center">
    <div class="w-1/3">Appearance</div>
    <div class="w-1/3">Allies and Factions</div>
    <div class="w-1/3">Traits ideals bonds and flaws</div>
  </div>
  <div class="flex text-center">
    <div class="w-1/3">Backstory</div>
    <div class="w-2/3">Notes</div>
  </div>
  <div>Spellcasting numbers</div>
  <div>Spells</div>
</main>
