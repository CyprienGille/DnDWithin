<script lang="ts">
	import { addSign } from './commons';
	import Proficiency from './Proficiency.svelte';
	import Roll from './Roll.svelte';

	export let skills;
</script>

<main class="variant-ghost-surface rounded content-center align-middle">
	{#each skills as skill}
		<div class="flex flex-row w-fit text-sm">
			<span class="w-4">
				<Proficiency proficiency={skill.proficiency} />
			</span>
			<span class="px-2">
				<Roll roll={skill.roll}>{addSign(skill.roll.bonus)}</Roll>
			</span>
			<span class="">
				{skill.name}
			</span>
		</div>
	{/each}
</main>
