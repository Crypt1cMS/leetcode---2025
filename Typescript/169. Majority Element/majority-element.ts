function majorityElement(nums: number[]): number {
    
    let myMap = new Map();
    let highestNum = 0;
    let result = null

    for (let num of nums) {
        myMap.set(num, (myMap.get(num) || 0) + 1)
    }

    for (let [key, value] of myMap) {
        if (value > highestNum) {
            highestNum = value
            result = key
        }
    }

    return result

};