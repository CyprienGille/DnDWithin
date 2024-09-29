<script lang="ts">
	import Abilities from './Abilities.svelte';

	export let c;
</script>

<div class="variant-ghost-surface rounded pl-2 pb-1">
	<h3 class="h3">{c.name}</h3>
	<div>{c.species}, {c.class}</div>
</div>
<Abilities abilities={c.abilities} st_fluff={c.st_fluff} />
