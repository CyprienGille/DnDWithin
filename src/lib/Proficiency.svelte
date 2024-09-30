<script lang="ts">
	import { mappingProf } from './commons';

	export let proficiency: string;

	let state = mappingProf.get(proficiency);

	function cycleState() {
		state = (state + 1) % 4;
		proficiency = mappingProf.revGet(state);
	}
</script>

<div title={proficiency}>
	{#if state == 0}
		<button class="p-1" on:click={cycleState}>N</button>
	{:else if state == 1}
		<button class="p-1" on:click={cycleState}>H</button>
	{:else if state == 2}
		<button class="p-1" on:click={cycleState}>P</button>
	{:else if state == 3}
		<button class="p-1" on:click={cycleState}>E</button>
	{/if}
</div>
