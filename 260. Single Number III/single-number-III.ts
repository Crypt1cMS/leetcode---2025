function singleNumber(nums: number[]): number[] {
    
    let result = []
    let myMap = new Map();

    for (let i = 0; i < nums.length; i++) {
        myMap.set(nums[i], (myMap.get(nums[i]) || 0) + 1)
    }

    for (let [key, value] of myMap) {
        if (value === 1) {
            result.push(key)
        }
    }

    return result
};