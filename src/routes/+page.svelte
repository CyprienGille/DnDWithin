<script lang="ts">
	import EditingSheet from '$lib/EditingSheet.svelte';
	import LockedSheet from '$lib/LockedSheet.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let c_promise: Promise<any> = invoke('get_character');

	let editing = true;
</script>

<main class="h-dvh w-screen">
	{#await c_promise}
		<h3 class="h3">Loading character...</h3>
	{:then c}
		{#if editing}
			<EditingSheet {c} />
		{:else}
			<LockedSheet {c} />
		{/if}
	{/await}
</main>
