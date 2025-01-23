function singleNumber(nums: number[]): number {

    let seenOnce = new Set<number>();
    let seenTwice = new Set<number>();

    for (let num of nums) {
        if (seenOnce.has(num)) {
            seenOnce.delete(num);
            seenTwice.add(num);
        } else if (!seenTwice.has(num)) {
            seenOnce.add(num)
        }
    }

    return [...seenOnce][0];

};