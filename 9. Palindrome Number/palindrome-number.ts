function isPalindrome(x: number): boolean {

    let xArr = x.toString().split("")
    let reversedArr = []

    for (let i = xArr.length - 1; i >= 0; i--) {
        reversedArr.push(xArr[i])
    }

    let originalNums = xArr.join("")
    let reversedNums = reversedArr.join("")

    return Number(originalNums) === Number(reversedNums)
};