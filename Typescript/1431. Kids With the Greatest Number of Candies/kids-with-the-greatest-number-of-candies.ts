function kidsWithCandies(candies: number[], extraCandies: number): boolean[] {

    let max = Math.max(...candies)
    return candies.map(candy => candy + extraCandies >= max)

};