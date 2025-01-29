function scoreOfString(s: string): number {
    
    let sSplit = s.split("")
    let numsArr = []

    for (let i = 0; i < sSplit.length - 1; i++) {
        let current = sSplit[i].charCodeAt(0)
        let nextVal = sSplit[i + 1].charCodeAt(0)

        numsArr.push(Math.abs(current - nextVal))
    }

    let result = numsArr.reduce((acc, next) => acc + next, 0)

    return result

};