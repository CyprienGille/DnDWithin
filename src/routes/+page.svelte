<script lang="ts">
	import EditingSheet from '$lib/EditingSheet.svelte';
	import menu from '$lib/assets/menu.svg';
	import { invoke } from '@tauri-apps/api/tauri';
	import { getDrawerStore, AppBar } from '@skeletonlabs/skeleton';

	const drawerStore = getDrawerStore();

	let c_promise: Promise<any> = invoke('get_character');

	let editing = true;
</script>

<main class="h-screen w-screen">
	<AppBar
		gridColumns="grid-cols-3"
		slotDefault="place-self-center"
		padding="px-4 py-3"
		slotTrail="place-content-end"
	>
		<svelte:fragment slot="lead">
			<button class="btn-icon btn-icon-sm" on:click={() => drawerStore.open()}>
				<img src={menu} class="dark:invert" alt="A hamburger menu icon" />
			</button>
		</svelte:fragment>
		<!-- (title) -->
		<svelte:fragment slot="trail">(actions)</svelte:fragment>
	</AppBar>

	{#await c_promise}
		<h3 class="h3">Loading character...</h3>
	{:then c}
		<EditingSheet {c} />
	{/await}
</main>
