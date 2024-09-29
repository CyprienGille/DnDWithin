<script lang="ts">
	import type { RollType } from './commons';
	import { add_sign } from './commons';

	export let roll: RollType;

	function get_bonus_string(bonus: number): string {
		if (bonus == 0) {
			return '';
		} else {
			return add_sign(bonus);
		}
	}

	function trigger_roll() {
		console.log('Rolled ' + roll.dice_amount + roll.dice + get_bonus_string(roll.bonus));
	}
</script>

<button class="btn btn-sm variant-ringed" on:click={trigger_roll}>
	<slot />
</button>
