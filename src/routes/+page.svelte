<script lang="ts">
	import Roll from '$lib/Roll.svelte';
	import { RollType } from '$lib/types';

	let requested_roll = new RollType();
	requested_roll.bonus = -10;
</script>

<main class="h-dvh w-screen">
	<div class="container h-full mx-auto flex justify-center items-center">
		<div class="space-y-5">
			<h1 class="h1">Let's get cracking bones!</h1>
			<Roll roll={requested_roll} />
		</div>
	</div>
</main>
