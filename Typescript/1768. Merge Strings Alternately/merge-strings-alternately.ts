function mergeAlternately(word1: string, word2: string): string {

    let wordLen = Math.max(word1.length, word2.length)
    let arr = []
    let result = ""

    for (let i = 0; i < wordLen; i++) {
        if (i % 2 === 0) {
            arr.push(word1[i])
            arr.push(word2[i])
        } else {
            arr.push(word1[i])
            arr.push(word2[i])
        }
    }

    for (let char of arr) {
        if (char !== undefined) {
            result += char
        }
    }

    return result


};