<script lang="ts">
	import Ability from './Ability.svelte';

	export let c;
</script>

<div class="h4">{c.name}</div>
<div>{c.species}, {c.class} {c.level}</div>
{#each c.abilities as ability}
	<Ability {ability} />
{/each}
