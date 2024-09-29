export class RollType {
    dice: string;
    dice_amount: number;
    bonus: number;
    
    constructor(dice:string, dice_amount:number, bonus:number) {
        this.dice = dice;
        this.dice_amount = dice_amount;
        this.bonus = bonus;
    }
}

export function add_sign(num: number): string {
    return (num >= 0 ? '+' : '') + num;
}