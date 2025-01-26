function checkIfPangram(sentence: string): boolean {

    let wordSplit = sentence.split("")
    let mySet = new Set(wordSplit);

    return mySet.size === 26

};