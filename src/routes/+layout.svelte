<script lang="ts">
	import '../app.postcss';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup, LightSwitch, initializeStores, Drawer } from '@skeletonlabs/skeleton';
	initializeStores();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Drawer width="w-64">
	<h1 class="h1 text-center mb-2">
		<span
			class="bg-gradient-to-br from-blue-900 to-cyan-300 bg-clip-text text-transparent box-decoration-clone"
			>DnDWithin</span
		>
	</h1>
	<div class="container grid-cols-1 w-full text-center">
		<button class="btn variant-soft-surface my-2">Open Character Sheet</button>
		<button
			class="btn variant-soft-surface my-2"
			title="Try filling all modifiers that use proficiencies.">Auto-fill modifiers</button
		>
		<!-- <button class="btn variant-soft-surface my-2">Close Character Sheet</button> -->
		<button class="btn variant-soft-surface my-2">Export to JSON</button>
		<div class="w-full justify-center flex my-2">
			<LightSwitch width="w-16" height="h-8" />
		</div>
	</div>
</Drawer>

<slot />
