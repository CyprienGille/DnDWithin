<script lang="ts">
	export let abilities;
	export let st_fluff = '';

	import { add_sign, RollType } from './commons';
	import Roll from './Roll.svelte';

	function scoreToMod(score: number): number {
		return Math.floor((score - 10) / 2);
	}
</script>

<div class="variant-ghost-surface rounded text-center grid grid-cols-2 w-80">
	{#each abilities as ability}
		<div class="px-2 py-1">
			<div title={ability.name}>{ability.short_name}</div>
			<div class="grid grid-rows-2 text-sm mb-1">
				<div class="grid grid-cols-3">
					<span title="Modifier">Mod</span>
					<span class="divider-vertical"></span>
					<span title="Saving throw modifier">Save</span>
				</div>
				<div class="grid grid-cols-3">
					<Roll roll={new RollType('d20', 1, scoreToMod(ability.score))}
						>{add_sign(scoreToMod(ability.score))}</Roll
					>
					<span class="divider-vertical"></span>
					<span title="Saving throw modifier">Save</span>
				</div>
			</div>
			<input
				class="input text-sm -my-0.5 text-center"
				title="Ability score"
				type="text"
				bind:value={ability.score}
			/>
		</div>
	{/each}
	<label class="label px-4 py-2 col-span-2">
		<span class="text-sm">Saving throws modifiers</span>
		<textarea class="textarea text-xs" rows="3" spellcheck="false" bind:value={st_fluff} />
	</label>
</div>
