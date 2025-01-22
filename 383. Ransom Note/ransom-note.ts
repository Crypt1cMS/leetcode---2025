function canConstruct(ransomNote: string, magazine: string): boolean {

    let myMapV1 = new Map()
    let myMapV2 = new Map()

    for (let char of ransomNote) {
        myMapV1.set(char, (myMapV1.get(char) || 0) + 1)
    }

    for (let char of magazine) {
        myMapV2.set(char, (myMapV2.get(char) || 0) + 1)
    }

    for (let [key, value] of myMapV1) {
        if (!myMapV2.has(key) || myMapV2.get(key)! < value) {
            return false
        } 
    }

    return true;

};