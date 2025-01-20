function missingNumber(nums: number[]): number {

    let sortedArr = nums.sort((a, b) => a - b);

    for (let i = 0; i <= sortedArr.length; i++) {
        if (sortedArr[i] !== i) {
            return i
        }
    }

    return 0
};