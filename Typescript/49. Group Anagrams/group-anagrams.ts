function groupAnagrams(strs: string[]): string[][] {

    let myMap: {[key: string]: string[]} = {}

    for (let str of strs) {
        const sortedKey = str.split("").sort().join("")

        if (!myMap[sortedKey]) {
            myMap[sortedKey] = []
        }
        
        myMap[sortedKey].push(str)
    }

    return Object.values(myMap)

};