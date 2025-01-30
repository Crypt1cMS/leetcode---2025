function letterCombinations(digits: string): string[] {

    let result: string[] = []
  
    if (digits === "") {
      return result
    }
  
    result.push("")
  
    const digitsObject: { [key: string]: string[] } = {
      2: ["a", "b", "c"],
      3: ["d", "e", "f"],
      4: ["g", "h", "i"],
      5: ["j", "k", "l"],
      6: ["m", "n", "o"],
      7: ["p", "q", "r", "s"],
      8: ["t", "u", "v"],
      9: ["w", "x", "y", "z"],
    };
  
    for (let digit of digits) {
      let letters = digitsObject[digit]
      let array: string[] = []
  
      for (let combinations of result) {
        for (let letter of letters) {
          array.push(combinations + letter)
        }
      }
  
      result = array
    }
  
    return result
  };