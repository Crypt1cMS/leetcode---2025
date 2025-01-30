function longestCommonPrefix(strs: string[]): string {

    let result = strs[0]

    for (let i = 1; i < strs.length; i++) {
        
        while(!strs[i].startsWith(result)) {
            result = result.slice(0, -1)
            if (result === "") return ""
        }

    }

    return strs.length === 0 ? "" : result

};