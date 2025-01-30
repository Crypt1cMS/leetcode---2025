function containsNearbyDuplicate(nums: number[], k: number): boolean {
    
    let mySet = new Set()

    for (let i = 0; i < nums.length; i++) {

        if (mySet.has(nums[i])) {
            return true
        }

        mySet.add(nums[i])

        if (mySet.size > k) {
            mySet.delete(nums[i - k])
        }

    }

    return false
};
