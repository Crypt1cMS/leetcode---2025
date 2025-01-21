function splitWordsBySeparator(words: string[], separator: string): string[] {
    
    let arrOfWords = words.map(str => str.split(separator))
    let result = []

    for (let i = 0; i < words.length; i++) {
        for (let word of arrOfWords[i]) {
            if (word.trim()) {
                result.push(word)
            }
        }
    }

    return result
};
