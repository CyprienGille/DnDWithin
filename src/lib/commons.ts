export class Dice {
    [key: string]:number;

    constructor(diceType:string, diceAmount:number) {
        this[diceType] = diceAmount;
    }
}

export function diceToString(dice:Dice):string {
    let out = "";
    for (const [key, value] of Object.entries(dice)) {
        out = value + key;
    }
    return out;
}

export class RollType {
    dice: Dice;
    bonus: number;
    reroll: string
    
    constructor(dice:Dice, bonus:number, reroll:string) {
        this.dice = dice;
        this.bonus = bonus;
        this.reroll = reroll;
    }
}

export function addSign(num: number): string {
    return (num >= 0 ? '+' : '') + num;
}

export class TwoWayStringToNum {
    map: { [x: string]: number; };
    reverseMap: { [x:number] : string};

    constructor(map: { [x: string]: number; }) {
       this.map = map;
       this.reverseMap = {};

       for(const key in map) {
          const value = map[key];
          this.reverseMap[value] = key;   
       }
    }

    get(key: string):number { return this.map[key]; }
    
    revGet(key: number):string { return this.reverseMap[key]; }
}

export const mappingProf = new TwoWayStringToNum({
    "NotProficient":0,
    "HalfProficient":1,
    "Proficient":2,
    "Expert":3,
})