function titleToNumber(columnTitle: string): number {
    
    let result = 0;

    const alphabet = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    for (let i = 0; i < columnTitle.length; i++) {
        
        let index = alphabet.indexOf(columnTitle[i]);
        result = result * 26 + (index + 1)
        
    }

    return result

};