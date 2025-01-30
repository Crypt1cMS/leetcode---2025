function maximumValue(strs: string[]): number {
    
    let result = 0;

    for (let i = 0; i < strs.length; i++) {
        const str = strs[i];
        let isNumeric = true;

        for (let j = 0; j < str.length; j++) {
            if (str[j] < '0' || str[j] > '9') {
                isNumeric = false
                break;
            }
        }

        if (isNumeric) {
            result = Math.max(result, parseInt(str, 10))
        } else {
            result = Math.max(result, str.length)
        }
    }

    return result

};