function numJewelsInStones(jewels: string, stones: string): number {
    
    let result = 0;
    let mySet = new Set(jewels)

    for (let stone of stones) {
        if (mySet.has(stone)) {
            result++
        }
    }

    return result

};