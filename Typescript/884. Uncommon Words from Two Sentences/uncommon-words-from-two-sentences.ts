function uncommonFromSentences(s1: string, s2: string): string[] {

    let words: string[] = s1.split(" ").concat(s2.split(" "));
    let myMap = new Map();
    let result: string[] = []

    for (let i = 0; i < words.length; i++) {
        myMap.set(words[i], (myMap.get(words[i]) || 0) + 1)
    }

    for (let [key, value] of myMap) {
        if (value === 1) {
            result.push(key)
        }
    }

    return result

    
};