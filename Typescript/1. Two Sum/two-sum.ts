function twoSum(nums: number[], target: number): number[] {

    let myMap = new Map();

    for (let i = 0; i < nums.length; i++) {
      let search = target - nums[i];

      if (myMap.has(search) ) {
        return [myMap.get(search), i]
      }
      
      myMap.set(nums[i], i)

    }
    
    return [];

};