<script lang="ts">
	export let proficiency: string;
</script>

<select class="select" bind:value={proficiency}>
	<option value="Not" />
	<option value="Half" />
	<option value="Proficient" />
	<option value="Expert" />
</select>
