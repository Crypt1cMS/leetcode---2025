function isBalanced(num: string): boolean {
    
    let arrNum = num.split("")
    let evenArr = []
    let oddArr = []

    for (let i = 0; i < arrNum.length; i++) {
        let strToNum = Number(arrNum[i])

        if (i % 2 === 0) {
            evenArr.push(strToNum)
        } else {
            oddArr.push(strToNum)
        }
    }

    let evenSum = evenArr.reduce((prev, curr) => prev + curr, 0)
    let oddSum = oddArr.reduce((prev, curr) => prev + curr, 0)

    return evenSum === oddSum

};