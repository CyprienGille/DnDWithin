<script lang="ts">
    export let c;
</script>

<main>
    <div class="w-full flex items-center py-5 bg-red-100">
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
                    <label for="classlevel" class="text-sm"
                        >Class and Level</label
                    >
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
    <div class="flex items-center">
        <div class="w-1/6 h-96 bg-red-300">ability scores</div>
        <div class="w-1/6 h-96 ">
            <div class="h-1/3 mb-2">
                saving throws saving throws saving throws saving throws saving
                throws
            </div>
            <div class="h-2/3 my-2">
                skills skills skills skills skills skills skills skills skills
            </div>
        </div>
        <div class="w-2/6 h-96 ">
            <div class="h-2/3 ">combat</div>
            <div class="h-1/3 ">actions</div>
        </div>
        <div class="w-2/6 h-96 ">hp and profs</div>
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
