function countPartitions(nums: number[]): number {
    
    let originalNums = [...nums];
    let arr = []
    let result = 0

    for (let i = 0; i < nums.length - 1; i++) {
        arr.push(nums[i])
        originalNums.shift()

        let sumOfArr = arr.reduce((acc, cur) => acc + cur, 0)
        let originalArr = originalNums.reduce((acc, cur) => acc + cur, 0)

        if ((sumOfArr - originalArr) % 2 === 0) {
            result++
        }
    }

    return result


};