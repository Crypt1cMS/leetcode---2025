function differenceOfSums(n: number, m: number): number {
    
    let notDivisible: number[] = []
    let divisible = []

    for (let i = 1; i <= n; i++) {

        if (i % m === 0) {
            divisible.push(i)
        } else {
            notDivisible.push(i)
        }

    }

    let notDivisibleSum = notDivisible.reduce((acc, val) => acc + val, 0)
    let divisibleSum = divisible.reduce((acc, val) => acc + val, 0)

    return notDivisibleSum - divisibleSum

};