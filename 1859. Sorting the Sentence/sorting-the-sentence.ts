function sortSentence(s: string): string {

    let sSplit = s.split(" ")
    let index = 0;
    let result = []

    for (let i = 0; i < sSplit.length; i++) {
        index++
        
        for (let j = 0; j < sSplit.length; j++) {
            if (sSplit[j].includes(index.toString())) {
                let newWord = sSplit[j].slice(0, sSplit[j].length - 1)
                result.push(newWord)
            }
        }

    }

    return result.join(" ")
};