<script lang="ts">
	import type { RollType } from './commons';
	import { addSign, diceToString } from './commons';

	export let roll: RollType;

	function get_bonus_string(bonus: number): string {
		if (bonus == 0) {
			return '';
		} else {
			return addSign(bonus);
		}
	}

	function trigger_roll() {
		console.log('Rolled ' + diceToString(roll.dice) + get_bonus_string(roll.bonus));
	}
</script>

<button class="btn btn-sm variant-ringed" on:click={trigger_roll}>
	<slot />
</button>
