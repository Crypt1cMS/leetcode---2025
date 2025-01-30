function isAnagram(s: string, t: string): boolean {
    let sStr = s.split("").sort().join("")
    let tStr = t.split("").sort().join("")

    return tStr === sStr
};