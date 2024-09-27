<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let c_promise: Promise<any> = invoke('get_character');
</script>

<main class="h-dvh w-screen">
	<div class="container h-full mx-auto flex justify-center items-center">
		<div class="space-y-5">
			<h1 class="h1">Let's get cracking bones!</h1>
			{#await c_promise}
				<h3 class="h3">Loading character...</h3>
			{:then c}
				<h3 class="h3">{c.name}</h3>
			{/await}
		</div>
	</div>
</main>
