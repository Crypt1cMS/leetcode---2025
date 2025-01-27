function maxAdjacentDistance(nums: number[]): number {

    let maxDiff = 0

    for (let i = 0; i < nums.length; i++) { 
        let diff = Math.abs((nums[i] - nums[(i + 1) % nums.length]))
        maxDiff = Math.max(maxDiff, diff)
    }

    return maxDiff

};