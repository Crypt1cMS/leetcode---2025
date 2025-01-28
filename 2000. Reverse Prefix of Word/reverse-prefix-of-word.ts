function prefixCount(words: string[], pref: string): number {

    let result = 0;

    for (let word of words) {
        if (word.startsWith(pref)) {
            result++
        }
    }

    return result

};