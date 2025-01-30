function decodeMessage(key: string, message: string): string {
    
    let myMap = new Map()
    let alphabet = "abcdefghijklmnopqrstuvwxyz"
    let index = 0

    for (let char of key) {
        if (char !== ' ' && !myMap.has(char) ) {
            myMap.set(char, alphabet[index])
            index++;
        }
    }

    return message.split('').map(char => (char === ' ' ? ' ' : myMap.get(char))).join('')

};