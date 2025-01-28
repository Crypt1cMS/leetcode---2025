function finalString(s: string): string {
    
    let sSplit = s.split("")
    let result = []

    for (let char of sSplit) {
        
        if (char !== "i") {
            result.push(char)
        } else {
            result.reverse()
        }

    }

    return result.join("")

};