function hammingWeight(n: number): number {
    
    let bit = 0

    while (n > 0) {
        if (n & 1) {
            bit++
        }

        n >>= 1;
    }

    return bit

};