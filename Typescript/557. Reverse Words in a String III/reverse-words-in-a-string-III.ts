function reverseWords(s: string): string {
    
    let words = s.split(" ");
    let reversedWords: string[] = [];


    for (let word of words) {
        let newWord = "";

        for (let i = word.length - 1; i >= 0; i--) {
            newWord += word[i]
        };

        reversedWords.push(newWord);
    }

    return reversedWords.join(" ")

};