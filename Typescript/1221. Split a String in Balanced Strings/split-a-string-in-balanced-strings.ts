function balancedStringSplit(s: string): number {

    let balance = 0;
    let result = 0;

    for (let char of s) {
        if (char === "L") {
            balance++
        } else {
            balance--;
        }

        if (balance === 0) {
            result++
        }
    }

    return result

};