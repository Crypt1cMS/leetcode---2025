function romanToInt(s: string): number {
    
    let myMap = new Map();
    let romanLetters: string[] = ["I", "V", "X", "L", "C", "D", "M"];
    let romanValues: number[] = [1, 5, 10, 50, 100, 500, 1000]
    let result: number = 0;

    for (let i = 0; i < romanLetters.length; i++) {
        myMap.set(romanLetters[i], romanValues[i])
    }

    for (let j = 0; j < s.length; j++) {
        const current = myMap.get(s[j])
        const next = myMap.get(s[j + 1])

        if (current < next) {
            result -= current
        } else {
            result += current
        }

    }

    return result

};