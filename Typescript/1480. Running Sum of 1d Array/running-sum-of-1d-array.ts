function runningSum(nums: number[]): number[] {
    
    let result = []
    let sum = 0

    for (let num of nums) {
        sum += num
        result.push(sum)
    }

    return result

};