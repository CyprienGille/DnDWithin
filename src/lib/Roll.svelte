<script lang="ts">
	import type { RollType } from './types';

	export let roll: RollType;

	function get_bonus_string(bonus: number): string {
		if (bonus == 0) {
			return '';
		} else if (bonus > 0) {
			return '+' + bonus;
		}
		return bonus.toString();
	}
</script>

<main>
	<span>{roll.dice_amount}{roll.dice}{get_bonus_string(roll.bonus)}</span>
</main>
