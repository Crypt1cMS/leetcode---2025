function lengthOfLastWord(s: string): number {

    let arrOfWords = s.trim().split(" ")
    let lengthOfArr = arrOfWords.length;
    return arrOfWords[lengthOfArr - 1].length
    
};