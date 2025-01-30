function intToRoman(num: number): string {
    let result = "";
    let myMap = new Map();
    const values: number[] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    const symbols: string[] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

    for (let i = 0; i < symbols.length; i++) {
        myMap.set(values[i], symbols[i])
    }

    for (let [key, value] of myMap) {
        while (num >= key) {
            result += value
            num -= key;
        }
    }

    return result
};