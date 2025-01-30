function singleNumber(nums: number[]): number {

    let myMap = new Map()

    for (let i = 0; i < nums.length; i++) {

        if (myMap.has(nums[i])) {
            myMap.set(nums[i], myMap.get(nums[i]) + 1)
        } else {
            myMap.set(nums[i], 1)
        }
        
    }

    for (let [key, value] of myMap) {
        
        if (value === 1) {
            return key
        }

    }

    return 0
    
};