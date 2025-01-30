function reverseBits(n: number): number {
    
    let binaryString = n.toString(2).padStart(32, "0");
    let reverse = binaryString.split("").reverse().join("");
    return parseInt(reverse, 2)

};