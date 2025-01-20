function searchInsert(nums: number[], target: number): number {

    if (nums.find(element => element === target)) {
      return nums.indexOf(target)
    } else {
      for (let i = 0; i < nums.length; i++) {
        if (nums[i] > target) {
          return i
        }
      }
    }
    return nums.length
  
  };
  