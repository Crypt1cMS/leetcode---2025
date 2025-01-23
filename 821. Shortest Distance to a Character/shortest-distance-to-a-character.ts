function shortestToChar(s: string, c: string): number[] {
    
    let arrOfNum = []
    let result = []

    for (let i = 0; i < s.length; i++) {
        if (s[i] === c) {
            arrOfNum.push(i)
        }
    }

    for (let i = 0; i < s.length; i++) {
        let distance: number = Math.abs(i - arrOfNum[0])

        for (let num of arrOfNum) {
            distance = Math.min(distance, Math.abs(i - num))
        }
        result.push(distance)
    }

    return result
};