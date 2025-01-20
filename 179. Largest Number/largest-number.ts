function largestNumber(nums: number[]): string {
    let sorted = nums.map(String).sort((a, b) => (b + a).localeCompare(a + b)).join("")

    if (sorted[0] === "0") return "0"

    return sorted
    
};